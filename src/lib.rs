#![crate_type = "lib"]
#![license = "MIT/ASL2"]
#![feature(globs, unsafe_destructor, phase)]

#[phase(plugin, link)] extern crate log;

extern crate libc;

extern crate libnanomsg;

use libc::{c_int};
use std::kinds::marker::ContravariantLifetime;
pub use result::{NanoResult, NanoError};
use result::{SocketInitializationError};

mod result;

#[deriving(Show, PartialEq)]
pub enum Protocol {
    Req,
    Rep,
    Push,
    Pull
}

/// A type-safe socket wrapper around nanomsg's own socket implementation. This
/// provides a safe interface for dealing with initializing the sockets, sending
/// and receiving messages.
pub struct Socket<'a> {
    addr: Option<&'a str>,
    socket: c_int,
    marker: ContravariantLifetime<'a>
}

impl<'a> Socket<'a> {
    pub fn new(protocol: Protocol) -> NanoResult<Socket<'a>> {

        let proto = match protocol {
            Req => libnanomsg::NN_REQ,
            Rep => libnanomsg::NN_REP,
            Push => libnanomsg::NN_PUSH,
            Pull => libnanomsg::NN_PULL
        };

        let socket = unsafe {
            libnanomsg::nn_socket(libnanomsg::AF_SP, proto)
        };

        if socket == -1 {
            return Err(NanoError::new("Failed to create a new nanomsg socket. Error: {}", SocketInitializationError));
        }

        Ok(Socket {
            addr: None,
            socket: socket,
            marker: ContravariantLifetime::<'a>
        })
    }
}

#[unsafe_destructor]
impl<'a> Drop for Socket<'a> {
    fn drop(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    extern crate debug;

}
