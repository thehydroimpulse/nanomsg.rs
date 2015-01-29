#![allow(unused_must_use)]

extern crate nanomsg;

use nanomsg::{Socket, Protocol};

use std::time::duration::Duration;
use std::old_io::timer::sleep;

const CLIENT_DEVICE_URL: &'static str = "ipc:///tmp/reqrep_example_front.ipc";
const SERVER_DEVICE_URL: &'static str = "ipc:///tmp/reqrep_example_back.ipc";

fn client() {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(CLIENT_DEVICE_URL).unwrap();
    let mut count = 1u32;

    let sleep_duration = Duration::milliseconds(100);

    loop {
        let request = format!("Request #{}", count);

        match socket.write(request.as_bytes()) {
            Ok(..) => println!("Send '{}'.", request.as_slice()),
            Err(err) => {
                println!("Client failed to send request '{}'.", err);
                break
            }
        }

        match socket.read_to_string() {
            Ok(reply) => println!("Recv '{}'.", reply.as_slice()),
            Err(err) => {
                println!("Client failed to receive reply '{}'.", err);
                break
            }
        }

        sleep(sleep_duration);
        count += 1;
    }

    endpoint.shutdown();
}

fn server() {
    let mut socket = Socket::new(Protocol::Rep).unwrap();
    let mut endpoint = socket.connect(SERVER_DEVICE_URL).unwrap();
    let mut count = 1u32;

    let sleep_duration = Duration::milliseconds(400);

    println!("Server is ready.");

    loop {

        match socket.read_to_string() {
            Ok(request) => {
                println!("Recv '{}'.", request.as_slice());

                let reply = format!("{} -> Reply #{}", request.as_slice(), count);
                match socket.write(reply.as_bytes()) {
                    Ok(..) => println!("Sent '{}'.", reply.as_slice()),
                    Err(err) => {
                        println!("Server failed to send reply '{}'.", err);
                        break
                    }
                }

                sleep(sleep_duration);
                count += 1;
            },
            Err(err) => {
                println!("Server failed to receive request '{}'.", err);
                break
            }
        }
    }

    endpoint.shutdown();
}

fn device() {
    let mut front_socket = Socket::new_for_device(Protocol::Rep).unwrap();
    let mut front_endpoint = front_socket.bind(CLIENT_DEVICE_URL).unwrap();
    let mut back_socket = Socket::new_for_device(Protocol::Req).unwrap();
    let mut back_endpoint = back_socket.bind(SERVER_DEVICE_URL).unwrap();

    println!("Device is ready.");
    Socket::device(&front_socket, &back_socket);
    println!("Device is stopped.");

    front_endpoint.shutdown();
    back_endpoint.shutdown();
}

fn usage() {
    println!("Usage: reqrep [client|server|device]");
    println!("  Try running several clients and servers");
    println!("  And also try killing and restarting them");
    println!("  Don't forget to start the device !");
}

fn main() {
    let args = std::os::args();

    if args.len() < 2 {
        return usage()
    }

    match args[1].as_slice() {
        "client" => client(),
        "server" => server(),
        "device" => device(),
        _ => usage()
    }
}
