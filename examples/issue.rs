extern crate nanomsg;

use nanomsg::{Protocol, Socket};
use std::thread;

use std::io::{Read, Write};

fn main() {
    let srv = thread::spawn(move || {
        println!("server: create socket");
        let mut s = Socket::new(Protocol::Pull).unwrap();
        println!("server: bind socket");
        //s.bind(&"inproc://test").unwrap();
        s.bind(&"tcp://127.0.0.1:5456").unwrap();

        println!("server: sleep 500");
        thread::sleep(std::time::Duration::from_millis(500));

        let mut msg = String::new();
        println!("server: recv");
        s.read_to_string(&mut msg).unwrap();
        println!("server: msg: {}", msg);
    });

    println!("client: sleep 100");
    // let the server start
    thread::sleep(std::time::Duration::from_millis(100));

    println!("client: create socket");
    let mut s = Socket::new(Protocol::Push).unwrap();
    println!("client: connect socket");
    //let mut ep = s.connect(&"inproc://test").unwrap();
    let mut ep = s.connect(&"tcp://127.0.0.1:5456").unwrap();
    println!("client: set_linger");
    s.set_linger(-1).expect("cannot set linger");
    println!("client: write_all");
    s.write_all("hello nanomsg".as_bytes()).unwrap();
    println!("client: shutdown");
    ep.shutdown().expect("cannot shutdown");
    println!("client: wait server");

    srv.join().expect("Can't join thread");
}
