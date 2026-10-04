use std::{
    println,
    thread::{self, JoinHandle, ThreadId},
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
