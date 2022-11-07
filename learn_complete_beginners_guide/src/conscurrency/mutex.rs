use std::{
    sync::{Arc, Mutex},
    thread,
};

///Mutex
///# Mutual exclusion lock
///
/// Only one thread can access the data any one time.
///
/// ## Arc
/// atomically referenced counted type.
/// convert data into primitive types, safe to shared across threads
/// ## Create a lock
/// ```
/// use std::sync:: {Mutex, Arc}
/// let lock = Arc::new(Mutex::new(0))
/// ```
///
/// ## Acquire a lock
/// `lock.lock()`
/// `lock.try_lock()`
///
/// ## Poisoned loc - when a thread that holds the lock panics
/// `lock.is_poisoned()`
pub fn mutex_fn() {
    let c = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for i in 0..10 {
        let c = Arc::clone(&c);
        let t = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        threads.push(t);
    }
    for j in threads {
        j.join();
    }
    println!("Result :{}", *c.lock().unwrap());
}
