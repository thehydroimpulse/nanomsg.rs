extern crate nanomsg;

use nanomsg::{Socket, Protocol};

fn worker() {
	let mut socket = Socket::new(Protocol::Pull).unwrap();
	let mut endpoint = socket.bind("ipc:///tmp/pipeline.ipc").unwrap();

	loop {
		let msg = socket.read_to_string().unwrap();

		println!("Worker received '{}'.", msg.as_slice());
	}
}

fn feeder() {
	let mut socket = Socket::new(Protocol::Push).unwrap();
	let mut endpoint = socket.connect("ipc:///tmp/pipeline.ipc").unwrap();

	socket.write(b"message in a bottle");

	endpoint.shutdown();
	drop(socket)
}

fn main() {
	let args = std::os::args();

	if args.len() < 2 {
		println!("Usage: pipeline worker, pipeline feeder")
		return
	}
	if args[1].as_slice() == "worker".as_slice() {
	    worker();
	}
	else if args[1].as_slice() == "feeder".as_slice() {
	    feeder();
	}
}