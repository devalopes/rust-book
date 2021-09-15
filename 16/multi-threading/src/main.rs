use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // Force a thread to stop execution for a short duration, allowing another to run
            // This thread will be stopped _whenever the main thread ends_
            // No guarantee on running order - or if this will be run at all...
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("\n\nNow with ensuring the thread completes");
    // Save value in variable
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Variable.join() returns a `JoinHandle` -
    // An owned value that will wait for spawned thread to finish.
    handle.join().unwrap();

    // Small details, such as where join is called, can affect whether or not your threads run at the same time.
    // The main thread will wait for the spawned thread to finish and then run its for loop, so the output won’t be interleaved anymore, as shown here:
    println!("\n\nNow with a moved join");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }


    // Using `move` to pass data into thread
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

}
