use std::{
    println,
    thread::{self, ThreadId},
};

fn main() {
    thread::spawn(test_spawn_thread);
    thread::spawn(test_spawn_thread);
    println!("Hello, world!");
}
fn test_spawn_thread() {
    println!("From another thread");

    let thread_id: ThreadId = thread::current().id();
    println!("Thread ID from function: {thread_id:?}");
}
