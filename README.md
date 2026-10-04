# Learning Rust Atomics and Multi-threaded

## Progress

- 1: Spawning threads
  thread::spawn(function) will spawj a thread and show their thread id (ThreadId)

- 2: Joining threads
  the join() method waits until the thread has finished executing and returns std::thread::Result. If thread hasn't successfully finished it's function because it's panicked, will contain the panic message.

- 3: Closures in thread spawn
