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
use std::io::{Writer, Reader, IoResult};
use std::io;

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
pub struct Socket {
    socket: c_int
}

impl Socket {

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
    pub fn new(protocol: Protocol) -> NanoResult<Socket> {

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
            socket: socket
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
    /// //match socket.bind("ipc:///tmp/pipeline.ipc") {
    /// //    Ok(_) => {},
    /// //   Err(err) => fail!("Failed to bind socket: {}", err)
    /// //}
    /// ```
    pub fn bind(&mut self, addr: &str) -> NanoResult<()> {
        let ret = unsafe { libnanomsg::nn_bind(self.socket, addr.to_c_str().as_ptr() as *const i8) };

        if ret == -1 {
            return Err(NanoError::new(format!("Failed to find the socket to the address: {}", addr), SocketBindError));
        }

        Ok(())
    }

    pub fn connect(&mut self, addr: &str) -> NanoResult<()> {
        let ret = unsafe { libnanomsg::nn_connect(self.socket, addr.to_c_str().as_ptr() as *const i8) };

        if ret == -1 {
            return Err(NanoError::new(format!("Failed to find the socket to the address: {}", addr), SocketBindError));
        }

        Ok(())
    }

}

impl Reader for Socket {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        let mut mem : *mut u8 = ptr::null_mut();

        let ret = unsafe {
            libnanomsg::nn_recv(self.socket, transmute(&mut mem),
                libnanomsg::NN_MSG, 0 as c_int)
        };

        if ret == -1 {
            return Err(io::standard_error(io::OtherIoError));
        }

        unsafe { ptr::copy_memory(buf.as_mut_ptr(), mem as *const u8, buf.len() as uint) };

        unsafe { libnanomsg::nn_freemsg(mem as *mut c_void) };

        Ok(ret as uint)
    }
}

impl Writer for Socket {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        let len = buf.len();
        let ret = unsafe {
            libnanomsg::nn_send(self.socket, buf.as_ptr() as *const c_void,
                                len as size_t, 0)
        };

        if ret as uint != len {
            return Err(io::standard_error(io::OtherIoError));
        }

        Ok(())
    }
}

#[unsafe_destructor]
impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { libnanomsg::nn_shutdown(self.socket, 0); }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    #[phase(plugin, link)]
    extern crate log;
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

    #[test]
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

            let mut buf = [0u8, ..6];
            match socket.read(&mut buf) {
                Ok(len) => {
                    assert_eq!(len, 6);
                    assert_eq!(buf.as_slice(), b"foobar")
                },
                Err(err) => fail!("{}", err)
            }

            drop(socket)
        });

        let mut socket = match Socket::new(Push) {
            Ok(socket) => socket,
            Err(err) => fail!("{}", err)
        };

        match socket.connect("ipc:///tmp/pipeline.ipc") {
            Ok(_) => {},
            Err(err) => fail!("{}", err)
        }

        match socket.write(b"foobar") {
            Ok(..) => {},
            Err(err) => fail!("Failed to write to the socket: {}", err)
        }
    }
}
