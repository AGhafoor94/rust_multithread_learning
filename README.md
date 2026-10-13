# Learning Rust Atomics and Multi-threaded

## Progress

### Spawning Threads

- 1: Spawning threads
  thread::spawn(function) will spawj a thread and show their thread id (ThreadId)

### Joining Threads

- 2: Joining threads
  the join() method waits until the thread has finished executing and returns std::thread::Result. If thread hasn't successfully finished it's function because it's panicked, will contain the panic message.

### Closures in Thread Spawn

- 3: Closures in thread spawn
  like thread::spawn(||{closure}).join().unwrap()
  thread::spawn(move || {closure}).join().unwrap()
  The ownership of variable is transferred to the new spawned thread because we used a move closure. if not used "move", it would have used variable by reference. The spawn function has a 'static lifetime bound on its argument type.

### Scoped Threads

- 4: Scoped threads
  If we know for sure that a spawned thread will definitely not outlive a certain scope, that thread could safely borrow things that don't live forever e.g. variables, as long as they outlive that scope.
  Use std::thread::scope to spawn scoped threads. allows us to spawn threads that can't outlive the scope of the closure we pass to that function.
  calling std::thread::scope with closure. closure is directly executed and gets an argument (scope in this case) representing the scope.
  We use scope to spawn scopes. The closure can borrow local variables like numbers_array
  When the scope ends, all threads that haven't been joined yet are automatically joined
  both threads spawned above access numbers_array but don't modify it

### Shared Ownership and Reference Counting (Rw)

- 5: Shared ownership and reference counting (Rw)
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

  dropping the rc will decrement the counter. Only the last Rc, which will see the counter drop to zero will be the one dropping and deallocating the contained data.
  sending an Rc to another thread would return a compiler error. use std::sync::Arc "atomically reference counted". Identical to Rc but guarantees that modifications to the reference
  counter are indivisible atomic operations, safe to use with multiple threads

### Borrowing and Data Races

- 6: Borrowing and data races:
  - Immutable borrowing:
    Borrowing something with & gives an immutable reference, which can be copied. Access to the data it references is shared between all copies of such a reference.

  - Mutable borrowing:
    Borrowing something with &mut gives a mutable reference. A mutable borrow guarantees it's the only active borrow of the data.

    These 2 together fully prevent data races: situations where one thread is mutating data while another is concurrently accessing it.
    Data races are generally undefined behaviour which means the compiler doesn't need to take these situations into account.

    example where the compiler can make a useful assumption using the borrowing rules: example_using_borrowing_rules
    we get an immutable reference to an integar and store the value of the integar both before and after incrementing the integar that b refers to.

### Interior Mutability

- 7: Interior Mutability

  A data type with interior mutability slightly bends the borrowing rules. Under certain conditions, those types can allow mutation through an "immutable" reference.

  In "Reference Counting (Rc)" and Arc mutate a reference counter even though there might be multiple clones all using the same reference counter
  As soon as interior mutable mutable types are involved, calling a reference "immutable" or "mutable" becomes confusing and inaccurate since both can be mutated through both.

  The more accurate terms are "shared" and "exclusive": a shared reference (&T) can be copied and shared with others, wile an exclusive reference (&mut T) guarantees it's the only exclusive borrowing of that T.
  For most types, shared references don't allow mutation, but there are exceptions.

  this only bends the riles of shared borrowing to allow mutation when shared. Doesn't change anything about exclusive borrowing. Exclusive borrowing still guarantees that there are no other active borrows.
  - (1) Cell:
    std::cell::Cell<T> wraps a T but allows mutations through a shared reference. To avoid undefined behaviour only allows you to copy the value out (if T is Copy) or replace it with another value as a whole.
    It can only be used within a single thread.

    now possible if condition to be true because Cell<i32> has interior mutability. both a and b might refer to the same value such that mutating through b might affect a. still may assume that no other threads are accessing the cells concurrently
    restrictions on Cell aren't always easy to work with. Can't directly let us borrow the value it holds. need to move a value out leaving something in its place, modify it and put it back to mutate the contents

  - (2) RefCell:
    std::cell::RefCell does allow you to borrow its contents at a small runtime cost. A RefCell<T> doesn't only hold a T but also holds a counter that keeps track of any outstanding borrows. If you try to borrow it while it's already
    mutably borrowed (or vice-versa), it will panic which avoids undefined behaviour. Can only be used in a single thread.
    Borrowing the contents of RefCell is done by calling borrow or borrow_mut

### Mutex and RwLock

- 8: Mutex and RwLock:
  An RwLock or reader-writer lock is the concurrent version of RefCell. An RwLock<T> holds a T and tracks any outstanding borrorws.

  unlike RefCell, it doesn't panic on conflicting borrows. It blocks the current thread - putting it to sleep - while waiting for conflicting borrows to disappear.

  We will have to wait for our turn with the data, after the other threads are done with it.

  Borrowing the contents of an RwLock is called locking. Locking it we temporarily block concurrent conflicting borrows, allowing us to borrow it without causing data races.

  A Mutex is similar but slightly simpler. Instead of keeping track of the number of shared and exclusive borrows like RwLock, it only allows exclusive borrows.

### Atomics

- 9: Atomics:
  The atomic types represent the concurrent version of a Cell. Like a Cell, they avoid undefined behaviour by making us copy values in and out as a whole without letting us borrow the contents directly.

  Unlike Cell, they cannot be of arbitrary size. Because of this, there isn't a generic Atomic<T> type for any T, but there are only specific atomic types such as:
  - AtomicU32
  - AtomicPtr<T>

  Since they are so limited in size, atomics often don't directly contain the information that needs to be shared between threads. They are often used as a tool to make it possible to share other - often bigger - things between threads. When atomics are used to say something about other data, things can get complicated.

### UnsafeCell

- 10: UnsafeCell:
  An UnsafeCell is the primitive building block for interior mutability

  UnsafeCell<T> wraps a T but doesn't come with any conditions or restrictions to avoid undefined behaviour. Its get() method just gives a raw pointer to the value it wraps. only used in unsafe{}. Leaves it up to the user to use in a a way that doesn't cause any undefined behaviour.

  Not used directly but wrapped in another type that provides safety through a limited interface e.g. Cell or Mutex. All types with interior mutabiltiy - including all types above are built on UnsafeCell.

### Thread Safety

#### Send and Sync

- 11: Thread Safety: Send and Sync:
  2 special traits to keep track of which types can be safely used across threads:
  - Send:
    A type is Send if it can be sent to another thread. If ownership of a value of that type can be transferred to another thread. Arc<u32> is send but Rc<u32> isn't.
  - Sync:
    A type is Sync if it can be shared with another thread. A type T is Sync if a shared reference to that type &T is Send. u32 is Sync but Cell<u32> isn't. Cell<u32> is Send.
    All primitive types e.g. i32, bool, str are both Send and Sync.
    Both of these traits are auto traits. Automatically implemented for types based on their fields. A struct with fields are all Send and Sync, it is also Send and Sync.

  Way to opt out of either of these is to add a field to the type that doesn't implement the trait. A special std::marker::PhantomData<T> type. That type is treated by the compiler as a T. Doesn't exist at runtime and is a zero-sized type.

  Function thread_safety_phantom_data has struct which can be both Send and Sync if handle was the only field. Added a zero-sized PhantomData<Cell<()>> field. This is treated as Cell<()>. Cell<()> isn't Sync. Still Send as all fields implement Send.

  The way to opt into either of the traits is the same way as any other traits. Using impl block. Need to add unsafe block. This is a promise to the compiler.

### Locking: Mutexes and RwLocks

- 11: Locking: Mutexes and RwLocks
  Most commonly used tool for sharing (mutable) data between threads is a mutex (mutual exclusion). Mutex is used to ensure threads have exlusive access to some data by temporarily blocking other threads that try to access it at the same time.

  Mutex only has 2 states. Locked and Unlocked.

  When a thread locks an unlocked mutex, the mutex is marked as locked and the thread can immediately continue. When a thread attempts to lock an already locked mutex, the operation will block. The thread is put to sleep while it waits for the mutex to be unlocked. Unlocking is only possible on a locked mutex and should only be done by the same thread that locked it. If other threads are waiting for the lock mutex, unlocking will cause one of the threads to be woken up so it can try to lock the mutex again.

  Protecting data with a mutex is the agreement between all threads that they will only access the data when they have the mutex locked.

### Rusts Mutex

- 12: Rusts Mutex:
  std provides functionality through std::sync::Mutex<T>. it's a generic over a type T which is the type of the data the mutex is protecting. Making T part of the mutex, the data can only be accessed through the mutex allowing for a safe interface that can guarantee all threads will uphold the agreement.

  Unlocking a locked mutex from it's original thread. it doesn't have unlock() but its lock() method returns a special type called a MutexGuard. This guard represents the guarantee that we have locked the mutex. Behaves like an exclusive reference through the DerefMut trait. Giving exclusive access to the data the mutex products. Unlocking the mutex is done by dropping the guard. Drop implementation of the guard will unlock the mutex.

  Function rust_mutex_locking_and_unlocking. A mutex protecting an integar, spawning 10 threads to each increment the integar 100 times. Each thread will first lock the mutex to obtain a MutexGuard and then use a guard to access the integar and mofify it. The guard increments 1 and then after scope is dropped. Take ownership after the scope by using into_inner().

### Lock Poisoning

- 13: Lock Poisoning:
  unwrap() relates to lock poisoning. A Mutex in Rust gets marked as poisoned when a thread panics while holding the lock. When that happens, the Mutex will no longer be locked, calling it's lock method will Err indicating a poisoned lock.

  Calling lock() on a poisoned mutex locks the mutex but Err returned by this is of type MutexGuard.
