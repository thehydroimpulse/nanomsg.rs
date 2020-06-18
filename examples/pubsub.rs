#![allow(unused_must_use)]

extern crate nanomsg;

use nanomsg::{Protocol, Socket};

use std::thread;
use std::time::Duration;

use std::io::{Read, Write};

const CLIENT_DEVICE_URL: &'static str = "ipc:///tmp/pubsub_example_front.ipc";
const SERVER_DEVICE_URL: &'static str = "ipc:///tmp/pubsub_example_back.ipc";

fn client(topic: &[u8]) {
    let mut socket = Socket::new(Protocol::Sub).unwrap();
    let setopt = socket.subscribe(topic);
    let mut endpoint = socket.connect(CLIENT_DEVICE_URL).unwrap();

    match setopt {
        Ok(_) => println!("Subscribed to '{:?}'.", topic),
        Err(err) => println!("Client failed to subscribe '{}'.", err),
    }

    let mut msg = String::new();
    loop {
        match socket.read_to_string(&mut msg) {
            Ok(_) => {
                println!("Recv '{}'.", msg);
                msg.clear()
            }
            Err(err) => {
                println!("Client failed to receive msg '{}'.", err);
                break;
            }
        }
    }

    endpoint.shutdown();
}

fn server(topic: &[u8]) {
    let mut socket = Socket::new(Protocol::Pub).unwrap();
    let mut endpoint = socket.connect(SERVER_DEVICE_URL).unwrap();
    let mut count = 1u32;

    println!("Server is ready.");

    let mut msg = Vec::with_capacity(topic.len() + 16);
    loop {
        let postfix = format!(" #{}", count);
        msg.clear();
        msg.extend_from_slice(topic);
        msg.extend_from_slice(postfix.as_bytes());
        match socket.write_all(&msg) {
            Ok(..) => println!("Published '{:?}'.", msg),
            Err(err) => {
                println!("Server failed to publish '{}'.", err);
                break;
            }
        }
        thread::sleep(Duration::from_millis(400));
        count += 1;
    }

    endpoint.shutdown();
}

fn device(topic: &[u8]) {
    let mut front_socket = Socket::new_for_device(Protocol::Pub).unwrap();
    let mut front_endpoint = front_socket.bind(CLIENT_DEVICE_URL).unwrap();
    let mut back_socket = Socket::new_for_device(Protocol::Sub).unwrap();
    let setopt = back_socket.subscribe(topic);
    let mut back_endpoint = back_socket.bind(SERVER_DEVICE_URL).unwrap();

    match setopt {
        Ok(_) => println!("Subscribed to '{:?}'.", topic),
        Err(err) => println!("Device failed to subscribe '{}'.", err),
    }

    println!("Device is ready.");
    Socket::device(&front_socket, &back_socket);
    println!("Device is stopped.");

    front_endpoint.shutdown();
    back_endpoint.shutdown();
}

fn usage() {
    println!("Usage: pubsub [client|server|device] topic");
    println!("  Try running several clients and servers");
    println!("  And also try killing and restarting them");
    println!("  Don't forget to start the device !");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        return usage();
    }

    match args[1].as_ref() {
        "client" => client(args[2].as_ref()),
        "server" => server(args[2].as_ref()),
        "device" => device(args[2].as_ref()),
        _ => usage(),
    }
}
