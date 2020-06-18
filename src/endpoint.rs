use libc::c_int;

use crate::result::{last_nano_error, Result};

/// An endpoint created for a specific socket. Each endpoint is identified
/// by a unique return value that can be further passed to a shutdown
/// function. The shutdown is done through the endpoint itself and not the Socket
pub struct Endpoint {
    value: c_int,
    socket: c_int,
}

impl Endpoint {
    pub fn new(value: c_int, socket: c_int) -> Endpoint {
        Endpoint { value, socket }
    }

    /// Removes an endpoint from the socket that created it (via `bind` or `connect`).
    /// The call will return immediately, however,
    /// the library will try to deliver any outstanding outbound messages to the endpoint
    /// for the time specified by `Socket::set_linger`.
    pub fn shutdown(&mut self) -> Result<()> {
        let ret = unsafe { nanomsg_sys::nn_shutdown(self.socket, self.value) };

        if ret == -1 {
            Err(last_nano_error())
        } else {
            Ok(())
        }
    }
}
