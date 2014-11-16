use libc::{c_int};
use std::kinds::marker::ContravariantLifetime;
use result::{NanoResult, last_nano_error};

use libnanomsg;

/// An endpoint created for a specific socket. Each endpoint is identified
/// by a unique return value that can be further passed to a shutdown
/// function. The shutdown is done through the endpoint itself and not
/// the Socket. However, the `Endpoint` doesn't live longer than the socket
/// itself. This is done through phantom lifetimes.
pub struct Endpoint<'a> {
    value: c_int,
    socket: c_int,
    marker: ContravariantLifetime<'a>
}

impl<'a> Endpoint<'a> {
    pub fn new(value: c_int, socket: c_int) -> Endpoint<'a> {
        Endpoint {
            value: value,
            socket: socket,
            marker: ContravariantLifetime::<'a>
        }
    }

    pub fn shutdown(&'a mut self) -> NanoResult<()> {
        let ret = unsafe { libnanomsg::nn_shutdown(self.socket, self.value) };

        if ret == -1 as c_int {
            return Err(last_nano_error());
        }

        Ok(())
    }
}
