use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use hello::ThreadPool;


/*
* -- IMPROVING THE DESIGN WITH A THREAD POOL --
* When a new task is received it will be assigned to a thread in the pool
* The remaining threads are available to process other tasks
* We will limit the number of possible threads in the pool to protect from Denial of Service
* E.g. if someone made 10 million requests it would eat all resources generating 10 million threads
*
* Fixed number of threads waiting in the pool
* Request comes in --> Sent to pool for processing
* Pool will maintain a queue of requests
* Each thread will pop off a request from the queue, handle the request, then ask for another
* Can handle 'N' requests concurrently doing this, where 'N' is number of threads available
* Alternative approach may be async IO (a library like tokio)
* (Disadvantage to asyncio is has to ship a larger runtime due to managing interrupts more)
*/

fn main() {
    // Transmission Control Protocol (TCP) lower level protocol describing _how_ information sent
    // Hypertext Transfer Protocol (HTTP) defines the _content_ which is sent on top of TCP

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            // Could be called multiple times due to browser requesting several resources (like favicon)
            handle_connection(stream);
        });

    }
}

fn handle_connection(mut stream: TcpStream) {
    // This buffer size should be managable for now
    // If we were handling arbitrary size requests we would need more complex buffer management
    let mut buffer = [0; 1024];

    // Basic HTTP request format
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body
    stream.read(&mut buffer).unwrap();

    // Define the method (GET) and endpoint (/) we accept requests on
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        // Simulating long processing time in a non-multi-threaded web server
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    // Basic HTTP Response format
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    // (Ex.) HTTP/1.1 200 OK\r\n\r\n
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
