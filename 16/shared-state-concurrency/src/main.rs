use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Using mutexes to allow access to data from one thread at a time
    // `mutex` abbreviates: mutual exclusion.
    // A thread must signal it wants to access the mutex by acquiring the mutexes lock
    // And then unlock when youre done so others can access it too.

    let m = Mutex::new(5);

    {
        // Panic if another thread already acquired the lock
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    // Lock is dropped here since mutex lock returns a smart pointer implementing Deref and Drop

    println!("m = {:?}", m);

    // Reference counter (rc) does not work since it does not implement `Send` trait
    // (Not safe to share accross threads)
    // Arc<T> (Atomic Reference Counter) is safe to use in concurrent situations
    // Atomics work like primitive types but are safe to share across threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Using this strategy, you can divide a calculation into independent parts,
    // split those parts across threads,
    // and then use a Mutex<T> to have each thread update the final result with its part.

    println!("Result: {}", *counter.lock().unwrap());
}
