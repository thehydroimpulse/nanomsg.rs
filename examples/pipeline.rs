#![allow(unused_must_use)]

extern crate nanomsg;

use nanomsg::{Protocol, Socket};

use std::string::String;
use std::thread;
use std::time::Duration;

use std::io::{Read, Write};

fn collector() {
    let mut socket = Socket::new(Protocol::Pull).unwrap();
    let mut text = String::new();
    socket.bind("ipc:///tmp/pipeline_collector.ipc");

    loop {
        match socket.read_to_string(&mut text) {
            Ok(_) => println!("Collected work result for '{}'.", text),
            Err(err) => {
                println!("Collector failed '{}'.", err);
                break;
            }
        }
        text.clear();
    }
}

fn worker() {
    let mut input = Socket::new(Protocol::Pull).unwrap();
    let mut output = Socket::new(Protocol::Push).unwrap();
    let mut msg = String::new();

    input.connect("ipc:///tmp/pipeline_worker.ipc");
    output.connect("ipc:///tmp/pipeline_collector.ipc");

    loop {
        match input.read_to_string(&mut msg) {
            Ok(_) => {
                println!("Worker received '{}'.", msg);

                thread::sleep(Duration::from_millis(300)); // fake some work ...
                output.write_all(msg.as_bytes());
                msg.clear();
            }
            Err(err) => {
                println!("Worker failed '{}'.", err);
                break;
            }
        }
    }
}

fn feeder() {
    let mut socket = Socket::new(Protocol::Push).unwrap();
    let mut endpoint = socket.bind("ipc:///tmp/pipeline_worker.ipc").unwrap();
    let mut count = 1u32;

    loop {
        let msg = format!("Message #{}", count);
        let msg_bytes = msg.as_bytes();
        match socket.write_all(msg_bytes) {
            Ok(_) => {
                thread::sleep(Duration::from_millis(100));
                count += 1;
            }
            Err(err) => {
                println!("Feeder failed '{}'.", err);
                break;
            }
        }
    }

    endpoint.shutdown();
}

fn usage() {
    println!("Usage: pipeline [feeder|worker|collector]");
    println!("  Try running several workers");
    println!("  And also try killing and restarting");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return usage();
    }

    match args[1].as_ref() {
        "worker" => worker(),
        "feeder" => feeder(),
        "collector" => collector(),
        _ => usage(),
    }
}
