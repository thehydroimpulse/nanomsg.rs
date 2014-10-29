#![feature(phase, globs, import_shadowing)]
#![allow(non_camel_case_types)]

#[phase(plugin)]
extern crate "link-config" as link_config;
extern crate libc;

use libc::{c_int, c_void, size_t, c_char};
use std::mem::transmute;

link_config!("libnanomsg", ["only_static"])

pub const AF_SP: c_int = 1;
pub const AF_SP_RAW: c_int = 2;
pub const NN_PROTO_PIPELINE: c_int = 5;
pub const NN_PUSH: c_int = NN_PROTO_PIPELINE * 16 + 0;
pub const NN_PULL: c_int = NN_PROTO_PIPELINE * 16 + 1;
pub const NN_MSG: u64 = -1;
pub const NN_PROTO_REQREP: c_int = 3;
pub const NN_REQ: c_int = NN_PROTO_REQREP * 16 + 0;
pub const NN_REP: c_int = NN_PROTO_REQREP * 16 + 1;
pub const NN_PROTO_PAIR: c_int = 1;
pub const NN_PAIR: c_int = NN_PROTO_PAIR * 16 + 0;

extern {
    /// "Creates an SP socket with specified domain and protocol. Returns
    /// a file descriptor for the newly created socket."
    ///
    /// http://nanomsg.org/v0.4/nn_socket.3.html
    pub fn nn_socket(domain: c_int, protocol: c_int) -> c_int;

    /// "Closes the socket s. Any buffered inbound messages that were not yet received
    /// by the application will be discarded. The library will try to deliver
    /// any outstanding outbound messages for the time specified by NN_LINGER socket
    /// option. The call will block in the meantime."
    ///
    /// http://nanomsg.org/v0.4/nn_close.3.html
    pub fn nn_close(socket: c_int) -> c_int;

    /// "Sets the value of the option. The level argument specifies the protocol level
    /// at which the option resides. For generic socket-level options use NN_SOL_SOCKET
    /// level. For socket-type-specific options use socket type for level argument
    /// (e.g. NN_SUB). For transport-specific options use ID of the transport as
    /// the level argument (e.g. NN_TCP).
    ///
    /// The new value is pointed to by optval argument. Size of the option is
    /// specified by the optvallen argument."
    ///
    /// http://nanomsg.org/v0.4/nn_setsockopt.3.html
    pub fn nn_setsockopt(socket: c_int, level: c_int, option: c_int, optval: *const c_void,
                         optvallen: size_t) -> c_int;

    /// "Retrieves the value for the option. The level argument specifies the protocol
    /// level at which the option resides. For generic socket-level options use NN_SOL_SOCKET
    /// level. For socket-type-specific options use socket type for level argument
    /// (e.g. NN_SUB). For transport-specific options use ID of the transport as the
    /// level argument (e.g. NN_TCP).
    ///
    /// The value is stored in the buffer pointed to by optval argument. Size of the
    /// buffer is specified by the optvallen argument. If the size of the option is greater
    /// than size of the buffer, the value will be silently truncated. Otherwise,
    /// the optvallen will be modified to indicate the actual length of the option."
    ///
    /// http://nanomsg.org/v0.4/nn_getsockopt.3.html
    pub fn nn_getsockopt(socket: c_int, level: c_int, option: c_int, optval: *mut c_void,
                         optvallen: size_t) -> c_int;
    /// "Adds a local endpoint to the socket s. The endpoint can be then used by other
    /// applications to connect to. The addr argument consists of two parts as follows:
    /// transport://address. The transport specifies the underlying transport protocol
    /// to use. The meaning of the address part is specific to the underlying transport
    /// protocol."
    ///
    /// http://nanomsg.org/v0.4/nn_bind.3.html
    pub fn nn_bind(socket: c_int, addr: *const c_char) -> c_int;

    /// "Adds a remote endpoint to the socket s. The library would then try to connect to the
    /// specified remote endpoint. The addr argument consists of two parts as follows:
    /// transport://address. The transport specifies the underlying transport protocol to use.
    /// The meaning of the address part is specific to the underlying transport protocol."
    ///
    /// http://nanomsg.org/v0.4/nn_connect.3.html
    pub fn nn_connect(socket: c_int, addr: *const c_char) -> c_int;

    /// "Removes an endpoint from socket s. how parameter specifies the ID of the endpoint to
    /// remove as returned by prior call to nn_bind(3) or nn_connect(3). nn_shutdown() call
    /// will return immediately, however, the library will try to deliver any outstanding
    /// outbound messages to the endpoint for the time specified by NN_LINGER socket option."
    ///
    /// http://nanomsg.org/v0.4/nn_shutdown.3.html
    pub fn nn_shutdown(socket: c_int, how: c_int) -> c_int;

    /// "The function will send a message containing the data from buffer pointed to by buf
    /// parameter to the socket s. The message will be len bytes long. Alternatively, to send
    /// a buffer allocated by nn_allocmsg(3) function set the buf parameter to point to the
    /// pointer to the buffer and len parameter to NN_MSG constant. In this case a successful
    /// call to nn_send will deallocate the buffer. Trying to deallocate it afterwards will
    /// result in undefined behaviour.
    ///
    /// Which of the peers the message will be sent to is determined by the particular socket
    /// type."
    ///
    /// http://nanomsg.org/v0.4/nn_send.3.html
    pub fn nn_send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> c_int;

    /// "Receive a message from the socket s and store it in the buffer referenced by the buf
    /// argument. Any bytes exceeding the length specified by the len argument will be truncated.
    ///
    /// Alternatively, nanomsg can allocate the buffer for you. To do so, let the buf parameter
    /// be a pointer to a void* variable (pointer to pointer) to the receive buffer and set the
    /// len parameter to NN_MSG. If the call is successful the user is responsible for
    /// deallocating the message using the nn_freemsg(3) function."
    ///
    /// http://nanomsg.org/v0.4/nn_recv.3.html
    pub fn nn_recv(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> c_int;

    /// http://nanomsg.org/v0.4/nn_sendmsg.3.html
    pub fn nn_sendmsg(socket: c_int, msghdr: *const c_void, flags: c_int) -> c_int;

    /// http://nanomsg.org/v0.4/nn_recvmsg.3.html
    pub fn nn_recvmsg(socket: c_int, msghdr: *mut c_void, flags: c_int) -> c_int;

    /// http://nanomsg.org/v0.4/nn_allocmsg.3.html
    pub fn nn_allocmsg(size: size_t, ty: c_int) -> *mut c_void;

    /// http://nanomsg.org/v0.4/nn_reallocmsg.3.html
    pub fn nn_reallocmsg(msg: *mut c_void, size: size_t) -> *mut c_void;

    /// http://nanomsg.org/v0.4/nn_freemsg.3.html
    pub fn nn_freemsg(msg: *mut c_void) -> c_int;

    /// http://nanomsg.org/v0.4/nn_poll.3.html
    pub fn nn_poll(fds: *mut c_void, nfds: c_int, timeout: c_int) -> c_int;

    /// http://nanomsg.org/v0.4/nn_errno.3.html
    pub fn nn_errno() -> c_int;

    /// http://nanomsg.org/v0.4/nn_strerror.3.html
    pub fn nn_strerror(errnum: c_int) -> *const c_char;
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::{c_void, c_int, c_char, size_t};
    use std::ptr;
    use std::mem::transmute;
    use std::string::raw::from_buf;

    /// This ensures that the one-way pipe works correctly and also serves as an example
    /// on how to properly use the low-level bindings directly, although it's recommended to
    /// use the high-level Rust idiomatic API to ensure safety. The low-level bindings are
    /// quite unsafe to use because there are a lot of unsafe pointers, unsafe blocks, etc...
    #[test]
    fn should_create_a_pipeline() {

        spawn(proc() {
            let url = "ipc:///tmp/pipeline.ipc".to_c_str();
            let mut sock = unsafe { nn_socket(AF_SP, NN_PULL) };

            assert!(sock >= 0);
            assert!(unsafe { nn_bind(sock, url.as_ptr()) } >= 0);

            loop {
                let mut buf: *mut u8 = ptr::null_mut();
                let bytes = unsafe { nn_recv(sock, transmute(&mut buf), NN_MSG, 0 as c_int) };
                assert!(bytes >= 0);
                let msg = unsafe { from_buf(buf as *const u8) };
                assert!(msg.as_slice() == "foobar");
                unsafe { nn_freemsg(buf as *mut c_void); }
                unsafe { nn_shutdown(sock, 0); }
                break;
            }
        });

        let url = "ipc:///tmp/pipeline.ipc".to_c_str();
        let mut sock = unsafe { nn_socket(AF_SP, NN_PUSH) };

        assert!(sock >= 0);
        assert!(unsafe { nn_connect(sock, url.as_ptr()) } >= 0);

        let msg = "foobar".to_c_str();
        let bytes = unsafe {
            nn_send(sock, msg.as_ptr() as *const c_void, msg.len() as size_t, 0)
        };

        assert!(bytes == 6);
        unsafe { nn_shutdown(sock, 0) };
    }
}
