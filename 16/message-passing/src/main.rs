//`mpsc` stands for multiple producer, single consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // One major tool Rust has for accomplishing message-sending concurrency is the channel,
    // a programming concept that Rust’s standard library provides an implementation of.
    // One part of your code calls methods on the transmitter with the data you want to send,
    // and another part checks the receiving end for arriving messages.
    // A channel is said to be closed if either the transmitter or receiver half is dropped.

    // Here, we’ll work up to a program that has one thread to generate values and send them down a channel,
    // and another thread that will receive the values and print them out.
    // tx = transmitter, rx = receiver
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    // We used `recv` here which blocks main execution until a value is received - and will then
    // return a Result<T,E>
    // When the transmitter closes, recv returns error to signal no more messages
    //
    // `try_recv` is another method that does NOT block execution, returns Result immediately
    // an Ok if there is a message and an Err if no messages at this time
    // Could write a loop to call this intermittently and handle message if one is available
    // ( reverse pub sub-esque)
    for received in rx {
        println!("Got: {}", received);
    }
}
