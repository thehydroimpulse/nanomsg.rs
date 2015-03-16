#![feature(core, std_misc, old_io)]
#![allow(unused_must_use)]

extern crate nanomsg;

use nanomsg::{Socket, Protocol};

use std::time::duration::Duration;
use std::old_io::timer::sleep;

use std::io::{Read, Write};

const CLIENT_DEVICE_URL: &'static str = "ipc:///tmp/pubsub_example_front.ipc";
const SERVER_DEVICE_URL: &'static str = "ipc:///tmp/pubsub_example_back.ipc";

fn client(topic: &str) {
    let mut socket = Socket::new(Protocol::Sub).unwrap();
    let mut endpoint = socket.connect(CLIENT_DEVICE_URL).unwrap();

    match socket.subscribe(topic) {
        Ok(_) => println!("Subscribed to '{}'.", topic.as_slice()),
        Err(err) => println!("Client failed to subscribe '{}'.", err)
    }

    let mut msg = String::new();
    loop {

        match socket.read_to_string(&mut msg) {
            Ok(_) => {
                println!("Recv '{}'.", msg.as_slice());
                msg.clear()
            },
            Err(err) => {
                println!("Client failed to receive msg '{}'.", err);
                break
            }
        }
    }

    endpoint.shutdown();
}

fn server(topic: &str) {
    let mut socket = Socket::new(Protocol::Pub).unwrap();
    let mut endpoint = socket.connect(SERVER_DEVICE_URL).unwrap();
    let mut count = 1u32;

    let sleep_duration = Duration::milliseconds(400);
    let mut request = String::new();

    println!("Server is ready.");

    loop {
        let msg = format!("{} #{}", topic,  count);
        match socket.write_all(msg.as_bytes()) {
            Ok(..) => println!("Published '{}'.", msg.as_slice()),
            Err(err) => {
                println!("Server failed to publish '{}'.", err);
                break
            }
        }
        sleep(sleep_duration);
        count += 1;
    }

    endpoint.shutdown();
}

fn device() {
    let mut front_socket = Socket::new_for_device(Protocol::Pub).unwrap();
    let mut front_endpoint = front_socket.bind(CLIENT_DEVICE_URL).unwrap();
    let mut back_socket = Socket::new_for_device(Protocol::Sub).unwrap();
    let mut back_endpoint = back_socket.bind(SERVER_DEVICE_URL).unwrap();

    back_socket.subscribe("");

    println!("Device is ready.");
    Socket::device(&front_socket, &back_socket);
    println!("Device is stopped.");

    front_endpoint.shutdown();
    back_endpoint.shutdown();
}

fn usage() {
    println!("Usage: pubsub [client topic|server topic|device]");
    println!("  Try running several clients and servers");
    println!("  And also try killing and restarting them");
    println!("  Don't forget to start the device !");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return usage()
    }

    match args[1].as_slice() {
        "client" => client(args[2].as_slice()),
        "server" => server(args[2].as_slice()),
        "device" => device(),
        _ => usage()
    }
}
