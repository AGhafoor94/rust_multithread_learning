use std::{
    println,
    thread::{self, JoinHandle, ThreadId},
};

fn main() {
    let thread_one: JoinHandle<()> = thread::spawn(test_spawn_thread);
    let thread_two: JoinHandle<()> = thread::spawn(test_spawn_thread);
    println!("Hello, world!");
    thread_one.join().unwrap();
    thread_two.join().unwrap();

    let numbers_in_array: [i16; 4] = [1, 2, 3, 4];

    // closures

    thread::spawn(move || {
        for num in numbers_in_array {
            println!("{num}");
        }
    })
    .join()
    .unwrap();

    // the above clouser. the ownership of numbers_in_array is transferred to the new spawned thread because we used a move closure. if not used "move", it would have used numbers_in_array by reference.
    // the spawn function has a 'static lifetime bound on its argument type.
}
fn test_spawn_thread() {
    println!("From another thread");

    let thread_id: ThreadId = thread::current().id();
    println!("Thread ID from function: {thread_id:?}");
}
