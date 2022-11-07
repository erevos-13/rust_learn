use std::{
    thread::{self, sleep},
    time::Duration,
};

///Threads
///# Run code in parallel.
///
/// Ownership/borrowing mechanism gives us
/// - memory safety
/// - no data races
///
/// ## Create a thread
/// ```
/// use std::thread
/// let th = thread::spawn(closure)
/// ```
///
/// ## Sleep a thread
/// ```
/// th::sleep()
/// ```
pub fn threads_fn() {
    //!# Threads loop
    //!
    //! If would like to use the vars from outer to inner thread we use::
    //! ```
    //! for i in 0..10 {
    //!  thread::spawn( move || println!("new thread! {}", i));
    //!}
    //! ```
    let mut threads = vec![];
    for i in 0..10 {
        sleep(Duration::from_millis(i * 1000));
        let th = thread::spawn(move || println!("new thread! {}", i));
        threads.push(th);
    }
    for t in threads {
        t.join();
    }
    println!("Main thread")
}
