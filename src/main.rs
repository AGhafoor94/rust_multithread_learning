use std::{
    assert_eq,
    cell::Cell,
    dbg, println,
    thread::{self, JoinHandle, ThreadId},
    vec,
};

fn main() {
    /*
        std::thread::spawn is shorthand for std::thread::Builder::new()::spawn().unwrap();
        std::thread::Builder allows you to set some settings for the new thread before spawning it. can configure the stack size for the new thread and to give the new thread a name. The name of a thread is available through std::thread::current().name()
        Builder's spawn function returns an std::io::Result. Can handle situations where spawning a new thread fails.
    */

    // Spawning and Joining threads
    join_threads();

    // Closures
    spawn_thread_closures();

    // Scoped threads
    scoped_threads();

    interior_mutability();
}
fn join_threads() {
    let thread_one: JoinHandle<()> = thread::spawn(test_spawn_thread);
    let thread_two: JoinHandle<()> = thread::spawn(test_spawn_thread);
    println!("Hello, world!");
    thread_one.join().unwrap();
    thread_two.join().unwrap();
}
fn test_spawn_thread() {
    println!("From another thread");

    let thread_id: ThreadId = thread::current().id();
    println!("Thread ID from function: {thread_id:?}");
}
fn spawn_thread_closures() {
    let numbers_in_array: [i16; 4] = [1, 2, 3, 4];

    // closures

    thread::spawn(move || {
        for num in numbers_in_array {
            println!("{num}");
        }
    })
    .join()
    .unwrap();

    /*
        the above clouser. the ownership of numbers_in_array is transferred to the new spawned thread because we used a move closure. if not used "move", it would have used numbers_in_array by reference.
        the spawn function has a 'static lifetime bound on its argument type.
    */

    // a vec array from 0 -> 1000 including 1000
    let numbers_vec: Vec<i32> = Vec::from_iter(0..=1000);

    let thread_return_after_move: JoinHandle<i32> = thread::spawn(move || {
        let length: i32 = numbers_vec.len() as i32;
        let sum: i32 = numbers_vec.iter().sum();
        sum / length
    });

    let average: i32 = thread_return_after_move.join().unwrap();
    println!("{average}");
}
fn scoped_threads() {
    /*
        If we know for sure that a spawned thread will definitely not outlive a certain scope, that thread could safely borrow things that don't live forever e.g. variables, as long as they outlive that scope.
        use std::thread::scope to spawn scoped threads. allows us to spawn threads that can't outlive the scope of the closure we pass to that function.
    */

    let numbers_array: [u16; 3] = [1, 2, 3];

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Lenght {}", numbers_array.len());
        });
        scope.spawn(|| {
            for num in &numbers_array {
                println!("Number {num}");
            }
        });
    });
    /*
        calling std::thread::scope with closure. closure is directly executed and gets an argument (scope in this case) representing the scope.
        we use scope to spawn scopes. The closure can borrow local variables like numbers_array
        when the scope ends, all threads that haven't been joined yet are automatically joined
        both threads spawned above access numbers_array but don't modify it

    */

    // error below, trying to access variable to edit in both thread scopes
    /*
        let mut vec_num_array: Vec<i32> = vec![1, 2, 3];

        thread::scope(|scope| {
            scope.spawn(|| {
                vec_num_array.push(1);
            });
            scope.spawn(|| {
                vec_num_array.push(2);
            });
        })
    */
}
fn shared_ownership_and_reference_counting() {
    /*
        Sharing data between threads without move (transferring owenership) and keeping value alive for longest thread

        (1) Statics:
                Several ways to create something that's not owned by a single thread. Simplest one is a static value. (1): both threads can access value but none own it.
                static item has a constant initialiser, never dropped and exists before main. Every thread can borrow it. Guaranteed to always exist.

        (2) Leaking:
                Leaking an allocation. Using Box::leak. Can release ownership of a Box, promising to never drop it. From that point onward, Box will live forever without an owner.
                Allowing it to be borrowed by any thread as long as the program runs

        (3) Reference Counting:
            To make sure that shared data gets dropped and deallocated, can share ownership, by keeping track of the number of owners, we can make sur the value is dropped only when there are no owners left.
            std::rc::Rc type short for "reference counted". Similar to Box but doesn't allcoate anything new. It increments a counter stored next to the contained value. Both the original and cloned Rc will refer
            to the same allocation they share ownership
    */

    // (1)

    static static_val: [u16; 4] = [1, 2, 3, 4];
    thread::spawn(|| dbg!(static_val));
    thread::spawn(|| dbg!(static_val));

    // (2)

    let leaking_val: &'static [i32; 4] = Box::leak(Box::new([1, 2, 3, 4]));

    thread::spawn(move || dbg!(leaking_val));
    thread::spawn(move || dbg!(leaking_val));

    // top we're only using reference not move by value. This is leaking memory and only runs to the end of the program

    // (3)

    let rc_a_value: std::rc::Rc<[i32; 3]> = std::rc::Rc::new([1, 2, 3]);
    let rc_b_value: std::rc::Rc<[i32; 3]> = rc_a_value.clone();

    assert_eq!(rc_a_value.as_ptr(), rc_b_value.as_ptr()); // same allocation

    /*
        dropping the rc will decrement the counter. Only the last Rc, which will see the counter drop to zero will be the one dropping and deallocating the contained data.
        sending an Rc to another thread would return a compiler error. use std::sync::Arc "atomically reference counted". Identical to Rc but guarantees that modifications to the reference
        counter are indivisible atomic operations, safe to use with multiple threads
    */

    // put an array in a new allocation together with the reference counter which starts at 1
    let atomic_rc_a_value: std::sync::Arc<[u32; 3]> = std::sync::Arc::new([1, 2, 3]);
    // cloning the Arc increments the count to two and provides us with a second Arc to the same allocation
    let atomic_rc_b_value: std::sync::Arc<[u32; 3]> = atomic_rc_a_value.clone();
    // both threads get their own Arc through which they can access the shared array. both decrement the reference counter when they drop their Arc.
    // the last thread to drop its Arc will see the counter drop to zero and will be the one to to drop and deallocated the array
    thread::spawn(move || dbg!(atomic_rc_a_value));
    thread::spawn(move || dbg!(atomic_rc_b_value));
}
fn borrowing_and_data_races() {
    /*
        Immutable borrowing:
            Borrowing something with & gives an immutable reference, which can be copied. Access to the data it references is shared between all copies of such a reference.

        Mutable borrowing:
            Borrowing something with &mut gives a mutable reference. A mutable borrow guarantees it's the only active borrow of the data.

        These 2 together fully prevent data races: situations where one thread is mutating data while another is concurrently accessing it.
        Data races are generally undefined behaviour which means the compiler doesn't need to take these situations into account.

    */

    let example_a_borrowing: i32 = 100;
    let mut example_b_borrowing: i32 = 200;
    example_using_borrowing_rules(&example_a_borrowing, &mut example_b_borrowing);
}
fn example_using_borrowing_rules(a: &i32, b: &mut i32) {
    /*
        example where the compiler can make a useful assumption using the borrowing rules: example_using_borrowing_rules
        we get an immutable reference to an integar and store the value of the integar both before and after incrementing the integar that b refers to.

    */
    let before: i32 = *a;

    *b += 1;
    let after: i32 = *a;

    if before != after {
        println!("Before not equals after") // never happens
    }
}
fn interior_mutability() {
    /*
        Interior Mutability:
            A data type with interior mutability slightly bends the borrowing rules. Under certain conditions, those types can allow mutation through an "immutable" reference.
            In "Reference Counting (Rc)" and Arc mutate a reference counter even though there might be multiple clones all using the same reference counter
            As soon as interior mutable mutable types are involved, calling a reference "immutable" or "mutable" becomes confusing and inaccurate since both can be mutated through both.
            The more accurate terms are "shared" and "exclusive": a shared reference (&T) can be copied and shared with others, wile an exclusive reference (&mut T) guarantees it's the only exclusive borrowing of that T.
            For most types, shared references don't allow mutation, but there are exceptions.

            this only bends the riles of shared borrowing to allow mutation when shared. Doesn't change anything about exclusive borrowing. Exclusive borrowing still guarantees that there are no other active borrows.

        (1) Cell:
                std::cell::Cell<T> wraps a T but allows mutations through a shared reference. To avoid undefined behaviour only allows you to copy the value out (if T is Copy) or replace it with another value as a whole.
                It can only be used within a single thread.

                now possible if condition to be true because Cell<i32> has interior mutability. both a and b might refer to the same value such that mutating through b might affect a. still may assume that no other threads are accessing the cells concurrently
                restrictions on Cell aren't always easy to work with. Can't directly let us borrow the value it holds. need to move a value out leaving something in its place, modify it and put it back to mutate the contents

        (2) RefCell:
                std::cell::RefCell does allow you to borrow its contents at a small runtime cost. A RefCell<T> doesn't only hold a T but also holds a counter that keeps track of any outstanding borrows. If you try to borrow it while it's already
                mutably borrowed (or vice-versa), it will panic which avoids undefined behaviour. Can only be used in a single thread.
                Borrowing the contents of RefCell is done by calling borrow or borrow_mut

    */

    // (1)

    let example_a_borrowing: std::cell::Cell<i32> = std::cell::Cell::new(100);
    let mut example_b_borrowing: std::cell::Cell<i32> = std::cell::Cell::new(200);
    example_using_cell(&example_a_borrowing, &mut example_b_borrowing);

    let example_cell_vec_mutate: std::cell::Cell<Vec<i32>> = std::cell::Cell::new(vec![100, 200]);
    example_using_cell_to_mutate(&example_cell_vec_mutate);

    // (2)

    let example_ref_cell_vec_mutate: std::cell::RefCell<Vec<i32>> =
        std::cell::RefCell::new(vec![1, 2, 3]);
    example_using_ref_cell(&example_ref_cell_vec_mutate);
}
fn example_using_cell(a: &std::cell::Cell<i32>, b: &mut std::cell::Cell<i32>) {
    let before: i32 = a.get();

    b.set(b.get() + 1);

    let after: i32 = a.get();

    if before != after {
        println!("Before not equals after"); // might happen
    }
}
fn example_using_cell_to_mutate(value: &std::cell::Cell<Vec<i32>>) {
    // replaces the contents of Cell with an empty vec

    let mut value_two: Vec<i32> = value.take();

    value_two.push(300);

    // put modified Vec back

    value.set(value_two);
}
fn example_using_ref_cell(value: &std::cell::RefCell<Vec<i32>>) {
    value.borrow_mut().push(200); // we can modify it directly
}
