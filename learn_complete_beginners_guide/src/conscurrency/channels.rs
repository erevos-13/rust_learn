use std::{sync::mpsc, thread, time::Duration};

///Channels
///
///# A way to send data between threads.
///
/// - MPSC , Multiple producer single receiver
///
/// ## Create a channel
/// ```
/// use std::sync::mpsc
/// let (tx, rx) = mpsc::channel
/// ```
///
/// ## Send message
/// `tx.send()`
///
/// ## Receive Message
/// ```
/// rx.recv() // blocking
/// rx.try_recv() //non blocking
/// ```
///
fn start_thread(d: usize, tx: mpsc::Sender<usize>) {
    thread::spawn(move || {
        println!("Settings the timer {}", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("seding the message {}", d);
        tx.send(d).unwrap();
    });
}
const NUM_THREADS: usize = 20;
pub fn channels_fn() {
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || tx.send(42).unwrap());
    // println!("receive: {}", rx.recv().unwrap())

    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_THREADS {
        start_thread(i, tx.clone());
    }
    for j in rx.iter().take(NUM_THREADS) {
        println!("receive: {} ", j);
    }
}
