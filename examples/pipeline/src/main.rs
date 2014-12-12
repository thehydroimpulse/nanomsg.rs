#![allow(unused_must_use)]

extern crate nanomsg;

use nanomsg::{Socket, Protocol};

use std::time::duration::Duration;
use std::io::timer::sleep;

fn collector() {
	let mut socket = Socket::new(Protocol::Pull).unwrap();
	
	socket.bind("ipc:///tmp/pipeline_collector.ipc");

	loop {
		match socket.read_to_string() {
			Ok(msg) => println!("Collected work result for '{}'.", msg.as_slice()),
			Err(err) => {
				println!("Collector failed '{}'.", err);
				break
			}
		}
	}
}

fn worker() {
	let mut input = Socket::new(Protocol::Pull).unwrap();
	let mut output = Socket::new(Protocol::Push).unwrap();
	
	input.connect("ipc:///tmp/pipeline_worker.ipc");
	output.connect("ipc:///tmp/pipeline_collector.ipc");

	loop {
		match input.read_to_string() {
			Ok(msg) => {
				println!("Worker received '{}'.", msg.as_slice());
				sleep(Duration::milliseconds(300)); // fake some work ...
				output.write(msg.as_bytes());
			},
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
	let sleep_duration = Duration::milliseconds(100);
	let mut count = 1u32;

	loop {
		let msg = format!("Message #{}", count);
		let msg_bytes = msg.as_bytes();
		let write_res = socket.write(msg_bytes);

		if write_res.is_err() {
			break;
		}

		sleep(sleep_duration);
		count = count + 1;
	}

	endpoint.shutdown();
}

fn main() {
	let args = std::os::args();

	if args.len() < 2 {
		println!("Usage: pipeline feeder, pipeline worker, pipeline collector")
		println!("  Try running several workers")
		println!("  And also try killing and restarting")
		return
	}
	if args[1].as_slice() == "worker".as_slice() {
	    worker();
	}
	else if args[1].as_slice() == "feeder".as_slice() {
	    feeder();
	}
	else if args[1].as_slice() == "collector".as_slice() {
	    collector();
	}
}