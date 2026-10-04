use std::{
    println,
    thread::{self, JoinHandle, ThreadId},
};

fn main() {
    /*
        std::thread::spawn is shorthand for std::thread::Builder::new()::spawn().unwrap();
    */

    let thread_one: JoinHandle<()> = thread::spawn(test_spawn_thread);
    let thread_two: JoinHandle<()> = thread::spawn(test_spawn_thread);
    println!("Hello, world!");
    thread_one.join().unwrap();
    thread_two.join().unwrap();
    spawn_thread_closures();
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
