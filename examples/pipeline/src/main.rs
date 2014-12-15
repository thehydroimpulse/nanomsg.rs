#![allow(unused_must_use)]

extern crate nanomsg;

use nanomsg::{Socket, Protocol};
use nanomsg::result::NanoResult;

fn worker() -> NanoResult<()> {
	let mut socket = try!(Socket::new(Protocol::Pull));
	let mut endpoint = try!(socket.bind("ipc:///tmp/pipeline.ipc"));

	loop {
		let msg = try!(socket.read_to_string());
		println!("Worker received '{}'.", &*msg);
	}

    Ok(())
}

fn feeder() -> NanoResult<()> {
	let mut socket = try!(Socket::new(Protocol::Push));
	let mut endpoint = try!(socket.connect("ipc:///tmp/pipeline.ipc"));

	socket.write(b"message in a bottle");
	endpoint.shutdown();
    Ok(())
}

fn main() {
    // Spawn the receiver
    spawn(proc() {
        match worker() {
            Ok(_) => {},
            Err(err) => panic!("The worker failed: {}", err)
        }
    });

    match feeder() {
        Ok(_) => {},
        Err(err) => panic!("The feeder has failed: {}", err)
    }
}
