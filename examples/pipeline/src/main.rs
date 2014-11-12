extern crate nanomsg;

use nanomsg::{Socket, Protocol};

fn server() {
	let mut socket = Socket::new(Protocol::Pull).unwrap();
	let mut endpoint = socket.bind("ipc:///tmp/pipeline.ipc").unwrap();

	loop {
		let msg = socket.read_to_string().unwrap();

		println!("Server received '{}'.", msg.as_slice());
	}

	endpoint.shutdown();
	drop(socket)
}

fn client() {
	let mut socket = Socket::new(Protocol::Push).unwrap();
	let mut endpoint = socket.connect("ipc:///tmp/pipeline.ipc").unwrap();

	socket.write(b"message in a bottle");

	endpoint.shutdown();
	drop(socket)
}

fn main() {
	let args = std::os::args();

	if args[1].as_slice() == "server".as_slice() {
	    server();
	}

	if args[1].as_slice() == "client".as_slice() {
	    client();
	}
}
