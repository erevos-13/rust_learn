use std::thread;

pub fn handle_thread() {
    let handle = thread::spawn(|| {
        println!("From a thread!");
    });
    println!("Before a thread");
    // Wait for execution to finish
    handle.join();
}
