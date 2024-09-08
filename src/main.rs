#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread::spawn,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // spawn a new thread to handle the connection
                spawn(move || {
                    println!("accepted new connection");
                    
                    let mut buf = [0;512];
                    stream.read(&mut buf).unwrap();

                    stream.write(b"+PONG\r\n").unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
