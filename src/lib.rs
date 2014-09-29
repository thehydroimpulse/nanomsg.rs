#![crate_type = "lib"]
#![license = "MIT/ASL2"]
#![feature(globs, unsafe_destructor, phase)]

#[phase(plugin, link)] extern crate log;

extern crate libc;

extern crate libnanomsg;

pub use result::{NanoResult, NanoError};

use libc::{c_int, c_void, size_t};
use std::mem::transmute;
use std::ptr;
use result::{SocketInitializationError, SocketBindError, SocketBufferError};

mod result;

/// Type-safe protocols that Nanomsg uses. Each socket
/// is bound to a single protocol that has specific behaviour
/// (such as only being able to receive messages and not send 'em).
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
    socket: c_int
}

impl<'a> Socket<'a> {

    /// Allocate and initialize a new Nanomsg socket which returns
    /// a new file descriptor behind the scene. The safe interface doesn't
    /// expose any of the underlying file descriptors and such.
    ///
    /// Usage:
    ///
    /// ```rust
    /// use nanomsg::{Socket, Pull};
    ///
    /// let mut socket = match Socket::new(Pull) {
    ///     Ok(socket) => socket,
    ///     Err(err) => fail!("{}", err)
    /// };
    /// ```
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

        debug!("Initialized a new raw socket");

        Ok(Socket {
            addr: None,
            socket: socket,
            marker: ContravariantLifetime::<'a>
        })
    }

    /// Creating a new socket through `Socket::new` does **not**
    /// bind that socket to a listening state. Instead, one has to be
    /// explicit in enabling the socket to listen onto a specific address.
    ///
    /// That's what the `bind` method does. Passing in a raw string like:
    /// "ipc:///tmp/pipeline.ipc" is supported.
    ///
    /// Note: This does **not** block the current task. That job
    /// is up to the user of the library by entering a loop.
    ///
    /// Usage:
    ///
    /// ```rust
    /// use nanomsg::{Socket, Pull};
    ///
    /// let mut socket = match Socket::new(Pull) {
    ///     Ok(socket) => socket,
    ///     Err(err) => fail!("{}", err)
    /// };
    ///
    /// // Bind the newly created socket to the following address:
    /// match socket.bind("ipc:///tmp/pipeline.ipc") {
    ///     Ok(_) => {},
    ///     Err(err) => fail!("Failed to bind socket: {}", err)
    /// }
    /// ```
    pub fn bind(&mut self, addr: &'a str) -> NanoResult<()> {
        let ret = unsafe { libnanomsg::nn_bind(self.socket, addr.to_c_str().as_ptr() as *const i8) };

        if ret == -1 {
            return Err(NanoError::new(format!("Failed to find the socket to the address: {}", addr), SocketBindError));
        }

        Ok(())
    }

    pub fn connect(&mut self, addr: &'a str) -> NanoResult<()> {
        let ret = unsafe { libnanomsg::nn_connect(self.socket, addr.to_c_str().as_ptr() as *const i8) };

        if ret == -1 {
            return Err(NanoError::new(format!("Failed to find the socket to the address: {}", addr), SocketBindError));
        }

        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> NanoResult<Vec<u8>> {
        let mut buf: *mut u8 = ptr::mut_null();

        let ret = unsafe {
            libnanomsg::nn_recv(self.socket, transmute(&mut buf),
                libnanomsg::NN_MSG, 0 as c_int)
        };

        if ret == -1 {
            return Err(NanoError::new("Failed to retrieve data from the socket", SocketBufferError));
        }

        Ok(unsafe { Vec::from_raw_parts(ret as uint, ret as uint, buf) })
    }

    pub fn write(&mut self, bytes: &[u8]) -> NanoResult<()> {
        let ret = unsafe {
            libnanomsg::nn_send(self.socket, bytes.as_ptr() as *const c_void,
                                bytes.len() as size_t, 0)
        };

        if ret as uint != bytes.len() {
            return Err(NanoError::new("Failed to write the buffer to the socket", SocketBufferError));
        }

        Ok(())
    }
}

#[unsafe_destructor]
impl<'a> Drop for Socket<'a> {
    fn drop(&mut self) {
        unsafe { libnanomsg::nn_shutdown(self.socket, 0); }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    #[phase(plugin, link)]
    extern crate log;
    extern crate debug;
    extern crate libnanomsg;
    extern crate libc;

    use super::*;

    use libc::{size_t, c_void};
    use std::string::raw::from_buf_len;

    #[test]
    fn initialize_socket() {
        let mut socket = match Socket::new(Pull) {
            Ok(socket) => socket,
            Err(err) => fail!("{}", err)
        };

        assert!(socket.socket >= 0);
    }

    #[test]
    fn bind_socket() {
        let mut socket = match Socket::new(Pull) {
            Ok(socket) => socket,
            Err(err) => fail!("{}", err)
        };

        match socket.bind("ipc:///tmp/pipeline.ipc") {
            Ok(_) => {},
            Err(err) => fail!("{}", err)
        }
    }

    fn receive_from_socket() {
        spawn(proc() {
            let mut socket = match Socket::new(Pull) {
                Ok(socket) => socket,
                Err(err) => fail!("{}", err)
            };


            match socket.bind("ipc:///tmp/pipeline.ipc") {
                Ok(_) => {},
                Err(err) => fail!("{}", err)
            }

            let mut buf = [0u8, ..8];
            match socket.read(&mut buf) {
                Ok(len) => println!("buf: {}", len),
                Err(err) => fail!("{}", err)
            }
        });

        let mut socket = match Socket::new(Push) {
            Ok(socket) => socket,
            Err(err) => fail!("{}", err)
        };

        match socket.connect("ipc:///tmp/pipeline.ipc") {
            Ok(_) => {},
            Err(err) => fail!("{}", err)
        }

        socket.write("foobar".as_bytes());
    }
}
