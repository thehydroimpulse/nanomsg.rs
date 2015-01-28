use libc::c_int;
use libnanomsg;
use result::{NanoResult,last_nano_error};
use std::marker::NoCopy;

/// An endpoint created for a specific socket. Each endpoint is identified
/// by a unique return value that can be further passed to a shutdown
/// function. The shutdown is done through the endpoint itself and not the Socket
pub struct Endpoint {
    value: c_int,
    socket: c_int,
    no_copy_marker: NoCopy
}

impl Endpoint {
    #[unstable]
    pub fn new(value: c_int, socket: c_int) -> Endpoint {
        Endpoint {
            value: value,
            socket: socket,
            no_copy_marker: NoCopy
        }
    }

    /// Removes an endpoint from the socket that created it (via `bind` or `connect`).
    /// The call will return immediately, however, 
    /// the library will try to deliver any outstanding outbound messages to the endpoint 
    /// for the time specified by `Socket::set_linger`.
    #[unstable]
    pub fn shutdown(&mut self) -> NanoResult<()> {

        let ret = unsafe { libnanomsg::nn_shutdown(self.socket, self.value) };

        if ret == -1 as c_int {
            Err(last_nano_error())
        } else {
            Ok(())
        }
    }
}
