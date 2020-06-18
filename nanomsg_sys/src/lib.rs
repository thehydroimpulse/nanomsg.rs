#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

pub use libc::*;

pub use posix_consts::*;

pub const AF_SP: c_int = 1;
pub const AF_SP_RAW: c_int = 2;
pub const NN_PROTO_PIPELINE: c_int = 5;
pub const NN_PUSH: c_int = NN_PROTO_PIPELINE * 16;
pub const NN_PULL: c_int = NN_PROTO_PIPELINE * 16 + 1;
pub const NN_MSG: size_t = std::usize::MAX as size_t;
pub const NN_PROTO_REQREP: c_int = 3;
pub const NN_REQ: c_int = NN_PROTO_REQREP * 16;
pub const NN_REP: c_int = NN_PROTO_REQREP * 16 + 1;
pub const NN_REQ_RESEND_IVL: c_int = 1;
pub const NN_PROTO_PAIR: c_int = 1;
pub const NN_PAIR: c_int = NN_PROTO_PAIR * 16;
pub const NN_PROTO_BUS: c_int = 7;
pub const NN_BUS: c_int = NN_PROTO_BUS * 16;
pub const NN_PROTO_PUBSUB: c_int = 2;
pub const NN_PUB: c_int = NN_PROTO_PUBSUB * 16;
pub const NN_SUB: c_int = NN_PROTO_PUBSUB * 16 + 1;
pub const NN_SUB_SUBSCRIBE: c_int = 1;
pub const NN_SUB_UNSUBSCRIBE: c_int = 2;
pub const NN_PROTO_SURVEY: c_int = 6;
pub const NN_SURVEYOR: c_int = NN_PROTO_SURVEY * 16 + 2;
pub const NN_RESPONDENT: c_int = NN_PROTO_SURVEY * 16 + 3;
pub const NN_SURVEYOR_DEADLINE: c_int = 1;


pub const NN_SOCKADDR_MAX: c_int = 128;

pub const NN_SOL_SOCKET: c_int = 0;

pub const NN_LINGER: c_int = 1;
pub const NN_SNDBUF: c_int = 2;
pub const NN_RCVBUF: c_int = 3;
pub const NN_SNDTIMEO: c_int = 4;
pub const NN_RCVTIMEO: c_int = 5;
pub const NN_RECONNECT_IVL: c_int = 6;
pub const NN_RECONNECT_IVL_MAX: c_int = 7;
pub const NN_SNDPRIO: c_int = 8;
pub const NN_RCVPRIO: c_int = 9;
pub const NN_SNDFD: c_int = 10;
pub const NN_RCVFD: c_int = 11;
pub const NN_DOMAIN: c_int = 12;
pub const NN_PROTOCOL: c_int = 13;
pub const NN_IPV4ONLY: c_int = 14;
pub const NN_SOCKET_NAME: c_int = 15;
pub const NN_RCVMAXSIZE: c_int = 16;
pub const NN_MAXTTL: c_int = 17;

pub const NN_DONTWAIT: c_int = 1;

pub const NN_INPROC: c_int = -1;
pub const NN_IPC: c_int = -2;
pub const NN_TCP: c_int = -3;

pub const NN_TCP_NODELAY: c_int = 1;

pub const NN_POLLIN: c_short = 1;
pub const NN_POLLOUT: c_short = 2;
pub const NN_POLL_IN_AND_OUT: c_short = NN_POLLIN + NN_POLLOUT;

// error codes
pub const ETERM: c_int = posix_consts::NN_HAUSNUMERO + 53;
pub const EFSM: c_int = posix_consts::NN_HAUSNUMERO + 54;

#[cfg(not(windows))]
pub mod posix_consts {
    use libc::*;
    // NOTE:
    // If the platform you are compiling for fails to implement the posix
    // constant, then add an exception below
    // If this fails to compile, then remove it from this re-export, and add
    // an exception below, similar to the macos exceptions already added. If
    // a feature is not implemented on your system, then use an offset to
    // NN_HAUSNUMERO, which nanomsg uses for undefined constants.
    //
    // Use the value from the windows definitions if an override is required.
    pub const NN_HAUSNUMERO: c_int = 156384712;

    // nanomsg uses EACCESS as an alias for EACCES
    pub const EACCESS: c_int = ::libc::EACCES;
}

#[cfg(windows)]
pub mod posix_consts {
    use libc::c_int;

    pub const NN_HAUSNUMERO: c_int = 156384712;

    pub const ENOTSUP:         c_int = NN_HAUSNUMERO + 1;
    pub const EPROTO:          c_int = NN_HAUSNUMERO + 11;
    pub const EACCESS:         c_int = NN_HAUSNUMERO + 17;
    pub const EISCONN:         c_int = NN_HAUSNUMERO + 27;
    pub const ESOCKTNOSUPPORT: c_int = NN_HAUSNUMERO + 28;

    pub const EADDRINUSE:      c_int = 100;
    pub const EADDRNOTAVAIL:   c_int = 101;
    pub const EAFNOSUPPORT:    c_int = 102;
    pub const ECONNABORTED:    c_int = 106;
    pub const ECONNREFUSED:    c_int = 107;
    pub const ECONNRESET:      c_int = 108;
    pub const EHOSTUNREACH:    c_int = 110;
    pub const EINPROGRESS:     c_int = 112;
    pub const EMSGSIZE:        c_int = 115;
    pub const ENETDOWN:        c_int = 116;
    pub const ENETRESET:       c_int = 117;
    pub const ENETUNREACH:     c_int = 118;
    pub const ENOBUFS:         c_int = 119;
    pub const ENOPROTOOPT:     c_int = 123;
    pub const ENOTCONN:        c_int = 126;
    pub const ENOTSOCK:        c_int = 128;
    pub const EPROTONOSUPPORT: c_int = 135;
    pub const ETIMEDOUT:       c_int = 138;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct nn_pollfd  {
    fd: c_int,
    events: c_short,
    revents: c_short
}

impl nn_pollfd {
    pub fn new (socket: c_int, pollin: bool, pollout: bool) -> nn_pollfd {
        let ev = match (pollin, pollout) {
            (true, true) => NN_POLL_IN_AND_OUT,
            (false, true) => NN_POLLOUT,
            (true, false) => NN_POLLIN,
            (false, false) => 0
        };

        nn_pollfd { fd: socket, events: ev , revents: 0 }
    }

    pub fn pollin_result(&self) -> bool {
        match self.revents {
            0 => false,
            NN_POLLIN => true,
            NN_POLLOUT => false,
            NN_POLL_IN_AND_OUT => true,
            _ => false
        }
    }

    pub fn pollout_result(&self) -> bool {
        match self.revents {
            0 => false,
            NN_POLLIN => false,
            NN_POLLOUT => true,
            NN_POLL_IN_AND_OUT => true,
           _ => false
         }
    }
}

#[cfg_attr(all(target_os = "linux", feature = "bundled", not(feature = "no_anl")), link(name = "anl"))]
#[cfg_attr(feature = "bundled", link(name = "nanomsg", kind = "static"))]
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
                         optvallen: *mut size_t) -> c_int;
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
    pub fn nn_poll(fds: *mut nn_pollfd, nfds: c_int, timeout: c_int) -> c_int;

    /// http://nanomsg.org/v0.4/nn_errno.3.html
    pub fn nn_errno() -> c_int;

    /// http://nanomsg.org/v0.4/nn_strerror.3.html
    pub fn nn_strerror(errnum: c_int) -> *const c_char;

    /// http://nanomsg.org/v0.4/nn_term.3.html
    pub fn nn_term() -> c_void;

    /// http://nanomsg.org/v0.4/nn_device.3.html
    pub fn nn_device(socket1: c_int, socket2: c_int) -> c_int;

    pub fn nn_symbol(index: c_int, value: *mut c_int) -> *const c_char;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice;
    use std::ptr;
    use std::mem::transmute;

    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::str;

    fn get_url(addr: &str) -> CString {
        CString::new(addr).unwrap()
    }

    fn sleep_ms(millis: u64) {
        thread::sleep(::std::time::Duration::from_millis(millis));
    }

    fn test_create_socket(domain: c_int, protocol: c_int) -> c_int {
        let sock = unsafe { nn_socket(domain, protocol) };
        assert!(sock >= 0);
        sock
    }

    fn test_bind(socket: c_int, addr: *const c_char) -> c_int {
        let endpoint = unsafe { nn_bind(socket, addr) };
        assert!(endpoint >= 0);
        endpoint
    }

    fn test_connect(socket: c_int, addr: *const c_char) -> c_int {
        let endpoint = unsafe { nn_connect(socket, addr) };
        assert!(endpoint >= 0);
        endpoint
    }

    fn test_send(socket: c_int, msg: &str) {
        let bytes = unsafe {
            nn_send(socket, msg.as_ptr() as *const c_void, msg.len() as size_t, 0)
        };
        let expected = msg.len() as i32;
        assert!(bytes == expected);
    }

    fn test_receive(socket: c_int, expected: &str) {
        let mut buf: *mut u8 = ptr::null_mut();
        let bytes = unsafe { nn_recv(socket, transmute(&mut buf), NN_MSG, 0) };
        assert!(bytes >= 0);
        let msg = unsafe { slice::from_raw_parts_mut(buf, bytes as usize) };
        assert_eq!(msg, expected.as_bytes());
        unsafe { nn_freemsg(buf as *mut c_void); }
    }

    fn test_subscribe(socket: c_int, topic: &str) {
        let topic_len = topic.len() as size_t;
        let topic_ptr = topic.as_ptr();
        let topic_raw_ptr = topic_ptr as *const c_void;
        assert!(unsafe { nn_setsockopt (socket, NN_SUB, NN_SUB_SUBSCRIBE, topic_raw_ptr, topic_len) } >= 0);
    }

    /// This ensures that the one-way pipe works correctly and also serves as an example
    /// on how to properly use the low-level bindings directly, although it's recommended to
    /// use the high-level Rust idiomatic API to ensure safety. The low-level bindings are
    /// quite unsafe to use because there are a lot of unsafe pointers, unsafe blocks, etc...
    #[test]
    fn should_create_a_pipeline() {

        let url = get_url("ipc:///tmp/should_create_a_pipeline.ipc");

        let push_sock = test_create_socket(AF_SP, NN_PUSH);
        let push_endpoint = test_bind(push_sock, url.as_ptr() as *const i8);

        let pull_sock = test_create_socket(AF_SP, NN_PULL);
        let pull_endpoint = test_connect(pull_sock, url.as_ptr() as *const i8);

        let push_msg = "foobar";
        test_send(push_sock, push_msg);
        test_receive(pull_sock, push_msg);

        unsafe {
            nn_shutdown(pull_sock, pull_endpoint);
            nn_close(pull_sock);
            nn_shutdown(push_sock, push_endpoint);
            nn_close(push_sock);
        }
    }

    #[test]
    fn should_create_a_pair() {

        let url = get_url("ipc:///tmp/should_create_a_pair.ipc");
        let left_sock = test_create_socket(AF_SP, NN_PAIR);
        let left_endpoint = test_bind(left_sock, url.as_ptr() as *const i8);

        let right_sock = test_create_socket(AF_SP, NN_PAIR);
        let right_endpoint = test_connect(right_sock, url.as_ptr() as *const i8);

        let right_to_left_msg = "foobar";
        test_send(right_sock, right_to_left_msg);
        test_receive(left_sock, right_to_left_msg);

        let left_to_right_msg = "foobaz";
        test_send(left_sock, left_to_right_msg);
        test_receive(right_sock, left_to_right_msg);

        unsafe {
            nn_shutdown(left_sock, left_endpoint);
            nn_close(left_sock);
            nn_shutdown(right_sock, right_endpoint);
            nn_close(right_sock);
        }
    }

    #[test]
    fn should_create_a_bus() {

        let url = get_url("ipc:///tmp/should_create_a_bus.ipc");

        let sock1 = test_create_socket(AF_SP, NN_BUS);
        let sock1_write_endpoint = test_bind(sock1, url.as_ptr() as *const i8);

        let sock2 = test_create_socket(AF_SP, NN_BUS);
        let sock2_read_endpoint = test_connect(sock2, url.as_ptr() as *const i8);

        let sock3 = test_create_socket(AF_SP, NN_BUS);
        let sock3_read_endpoint = test_connect(sock3, url.as_ptr() as *const i8);

        sleep_ms(10);

        let msg = "foobar";
        test_send(sock1, msg);
        test_receive(sock2, msg);
        test_receive(sock3, msg);

        unsafe {
            nn_shutdown(sock3, sock3_read_endpoint);
            nn_shutdown(sock2, sock2_read_endpoint);
            nn_shutdown(sock1, sock1_write_endpoint);

            nn_close(sock3);
            nn_close(sock2);
            nn_close(sock1);
        }
    }

    #[test]
    fn should_create_a_pubsub() {

        let url = get_url("ipc:///tmp/should_create_a_pubsub.ipc");
        let pub_sock = test_create_socket(AF_SP, NN_PUB);
        let pub_endpoint = test_bind(pub_sock, url.as_ptr() as *const i8);

        let sub_sock1 = test_create_socket(AF_SP, NN_SUB);
        let sub_endpoint1 = test_connect(sub_sock1, url.as_ptr() as *const i8);
        let topic1 = "foo";
        test_subscribe(sub_sock1, topic1);

        let sub_sock2 = test_create_socket(AF_SP, NN_SUB);
        let sub_endpoint2 = test_connect(sub_sock2, url.as_ptr() as *const i8);
        let topic2 = "bar";
        test_subscribe(sub_sock2, topic2);

        sleep_ms(100);

        let msg1 = "foobar";
        test_send(pub_sock, msg1);
        test_receive(sub_sock1, msg1);

        let msg2 = "barfoo";
        test_send(pub_sock, msg2);
        test_receive(sub_sock2, msg2);

        unsafe {
            nn_shutdown(sub_sock2, sub_endpoint2);
            nn_shutdown(sub_sock1, sub_endpoint1);
            nn_shutdown(pub_sock, pub_endpoint);

            nn_close(sub_sock2);
            nn_close(sub_sock1);
            nn_close(pub_sock);
        }
    }

    #[test]
    fn should_create_a_survey() {

        let url = get_url("ipc:///tmp/should_create_a_survey.ipc");
        let surv_sock = test_create_socket(AF_SP, NN_SURVEYOR);
        let surv_endpoint = test_bind(surv_sock, url.as_ptr() as *const i8);

        let resp_sock1 = test_create_socket(AF_SP, NN_RESPONDENT);
        let resp_endpoint1 = test_connect(resp_sock1, url.as_ptr() as *const i8);

        let resp_sock2 = test_create_socket(AF_SP, NN_RESPONDENT);
        let resp_endpoint2 = test_connect(resp_sock2, url.as_ptr() as *const i8);

        sleep_ms(10);

        let survey = "are_you_there";
        test_send(surv_sock, survey);
        test_receive(resp_sock1, survey);
        test_receive(resp_sock2, survey);

        let vote = "yes";
        test_send(resp_sock1, vote);
        test_send(resp_sock2, vote);
        test_receive(surv_sock, vote);
        test_receive(surv_sock, vote);

        unsafe {
            nn_shutdown(resp_sock2, resp_endpoint2);
            nn_shutdown(resp_sock1, resp_endpoint1);
            nn_shutdown(surv_sock, surv_endpoint);

            nn_close(resp_sock2);
            nn_close(resp_sock1);
            nn_close(surv_sock);
        }
    }

    #[test]
    fn poll_should_work() {
        let url = get_url("ipc:///tmp/poll_should_work.ipc");
        let s1 = test_create_socket(AF_SP, NN_PAIR);
        let s2 = test_create_socket(AF_SP, NN_PAIR);
        let pollfd1 = nn_pollfd { fd: s1, events: 3i16, revents: 0i16 };
        let pollfd2 = nn_pollfd { fd: s2, events: 3i16, revents: 0i16 };
        let mut fd_vector: Vec<nn_pollfd> = vec![pollfd1, pollfd2];
        let fd_ptr = fd_vector.as_mut_ptr();

        let poll_result = unsafe { nn_poll(fd_ptr, 2, 0) as usize };
        let fd_slice = &mut fd_vector[..];
        assert_eq!(0, poll_result);
        assert_eq!(0, fd_slice[0].revents);
        assert_eq!(0, fd_slice[1].revents);

        test_bind(s1, url.as_ptr() as *const i8);
        test_connect(s2, url.as_ptr() as *const i8);
        sleep_ms(10);

        let poll_result2 = unsafe { nn_poll(fd_ptr, 2, 10) as usize };
        assert_eq!(2, poll_result2);
        assert_eq!(NN_POLLOUT, fd_slice[0].revents);
        assert_eq!(NN_POLLOUT, fd_slice[1].revents);

        let msg = "foobar";
        test_send(s2, msg);
        sleep_ms(10);

        let poll_result3 = unsafe { nn_poll(fd_ptr, 2, 10) as usize };
        assert_eq!(2, poll_result3);
        assert_eq!(NN_POLLOUT + NN_POLLIN, fd_slice[0].revents);
        assert_eq!(NN_POLLOUT, fd_slice[1].revents);

        unsafe {
            nn_close(s1);
            nn_close(s2);
        }
    }

    fn finish_child_task(checkin: Arc<Barrier>, socket: c_int, endpoint: c_int) {

        checkin.wait();

        unsafe {
            nn_shutdown(socket, endpoint);
            nn_close(socket);
        }
    }

    fn test_multithread_pipeline(url: String) {

        // this is required to prevent the sender from being closed before the receiver gets the message
        let drop_after_use = Arc::new(Barrier::new(2));
        let drop_after_use_pull = drop_after_use.clone();
        let drop_after_use_push = drop_after_use.clone();
        let push_url = get_url(&url);
        let pull_url = get_url(&url);

        let push_thread = thread::spawn(move || {
            let push_msg = "foobar";
            let push_sock = test_create_socket(AF_SP, NN_PUSH);
            let push_endpoint = test_bind(push_sock, push_url.as_ptr() as *const i8);

            test_send(push_sock, push_msg);

            finish_child_task(drop_after_use_push, push_sock, push_endpoint);
        });

        let pull_thread = thread::spawn(move || {
            let pull_msg = "foobar";
            let pull_sock = test_create_socket(AF_SP, NN_PULL);
            let pull_endpoint = test_connect(pull_sock, pull_url.as_ptr() as *const i8);

            test_receive(pull_sock, pull_msg);

            finish_child_task(drop_after_use_pull, pull_sock, pull_endpoint);
        });

        push_thread.join().unwrap();
        pull_thread.join().unwrap();
    }

    #[test]
    fn should_create_a_pipeline_mt1() {
        test_multithread_pipeline(String::from("ipc:///tmp/should_create_a_pipeline_mt1.ipc"))
    }

    #[test]
    fn should_create_a_pipeline_mt2() {
        test_multithread_pipeline(String::from("ipc:///tmp/should_create_a_pipeline_mt2.ipc"))
    }

    #[test]
    fn constants_should_match_return_of_symbol_func() {
        unsafe {
            let mut index: c_int = 0;
            loop {
                let mut c_value: c_int = -1;
                let c_name_ptr = nn_symbol(index, &mut c_value);

                if c_name_ptr.is_null() {
                   break;
                }

                let c_name_str = CStr::from_ptr(c_name_ptr);
                let c_name_bytes = c_name_str.to_bytes();
                let c_name = str::from_utf8(c_name_bytes).unwrap();
                let mb_rust_value = get_constant_value_by_name(c_name);

                if mb_rust_value.is_some() {
                    let rust_value = mb_rust_value.unwrap();
                    if c_value != rust_value {
                        panic!("Constant {} value mismatch: {} != {}", c_name, c_value, rust_value);
                    }
                }

                index = index + 1;
            }
        }
    }

    fn get_constant_value_by_name(name: &str) -> Option<c_int> {
        match name {
            "AF_SP" => Some(AF_SP),
            "AF_SP_RAW" => Some(AF_SP_RAW),
            "NN_PROTO_PIPELINE" => Some(NN_PROTO_PIPELINE),
            "NN_PUSH" => Some(NN_PUSH),
            "NN_PULL" => Some(NN_PULL),
            "NN_PROTO_REQREP" => Some(NN_PROTO_REQREP),
            "NN_REQ" => Some(NN_REQ),
            "NN_REP" => Some(NN_REP),
            "NN_REQ_RESEND_IVL" => Some(NN_REQ_RESEND_IVL),
            "NN_PROTO_PAIR" => Some(NN_PROTO_PAIR),
            "NN_PAIR" => Some(NN_PAIR),
            "NN_PROTO_BUS" => Some(NN_PROTO_BUS),
            "NN_BUS" => Some(NN_BUS),
            "NN_PROTO_PUBSUB" => Some(NN_PROTO_PUBSUB),
            "NN_PUB" => Some(NN_PUB),
            "NN_SUB" => Some(NN_SUB),
            "NN_SUB_SUBSCRIBE" => Some(NN_SUB_SUBSCRIBE),
            "NN_SUB_UNSUBSCRIBE" => Some(NN_SUB_UNSUBSCRIBE),
            "NN_PROTO_SURVEY" => Some(NN_PROTO_SURVEY),
            "NN_SURVEYOR" => Some(NN_SURVEYOR),
            "NN_RESPONDENT" => Some(NN_RESPONDENT),
            "NN_SURVEYOR_DEADLINE" => Some(NN_SURVEYOR_DEADLINE),
            "NN_SOCKADDR_MAX" => Some(NN_SOCKADDR_MAX),
            "NN_SOL_SOCKET" => Some(NN_SOL_SOCKET),
            "NN_LINGER" => Some(NN_LINGER),
            "NN_SNDBUF" => Some(NN_SNDBUF),
            "NN_RCVBUF" => Some(NN_RCVBUF),
            "NN_SNDTIMEO" => Some(NN_SNDTIMEO),
            "NN_RCVTIMEO" => Some(NN_RCVTIMEO),
            "NN_RECONNECT_IVL" => Some(NN_RECONNECT_IVL),
            "NN_RECONNECT_IVL_MAX" => Some(NN_RECONNECT_IVL_MAX),
            "NN_SNDPRIO" => Some(NN_SNDPRIO),
            "NN_RCVPRIO" => Some(NN_RCVPRIO),
            "NN_SNDFD" => Some(NN_SNDFD),
            "NN_RCVFD" => Some(NN_RCVFD),
            "NN_DOMAIN" => Some(NN_DOMAIN),
            "NN_PROTOCOL" => Some(NN_PROTOCOL),
            "NN_IPV4ONLY" => Some(NN_IPV4ONLY),
            "NN_SOCKET_NAME" => Some(NN_SOCKET_NAME),
            "NN_RCVMAXSIZE" => Some(NN_RCVMAXSIZE),
            "NN_DONTWAIT" => Some(NN_DONTWAIT),
            "NN_INPROC" => Some(NN_INPROC),
            "NN_IPC" => Some(NN_IPC),
            "NN_TCP" => Some(NN_TCP),
            "NN_TCP_NODELAY" => Some(NN_TCP_NODELAY),
            "ETERM" => Some(ETERM),
            "EFSM" => Some(EFSM),
            "ENAMETOOLONG" => Some(ENAMETOOLONG),
            "ENODEV" => Some(ENODEV),
            "EINTR" => Some(EINTR),
            "NN_HAUSNUMERO" => Some(NN_HAUSNUMERO),
            "ENOTSUP " => Some(ENOTSUP ),
            "EPROTONOSUPPORT" => Some(EPROTONOSUPPORT),
            "ENOBUFS" => Some(ENOBUFS),
            "ENETDOWN" => Some(ENETDOWN),
            "EADDRINUSE" => Some(EADDRINUSE),
            "EADDRNOTAVAIL" => Some(EADDRNOTAVAIL),
            "ECONNREFUSED" => Some(ECONNREFUSED),
            "EINPROGRESS" => Some(EINPROGRESS),
            "ENOTSOCK" => Some(ENOTSOCK),
            "EAFNOSUPPORT" => Some(EAFNOSUPPORT),
            "EPROTO " => Some(EPROTO ),
            "EAGAIN" => Some(EAGAIN),
            "EBADF" => Some(EBADF),
            "EINVAL" => Some(EINVAL),
            "EMFILE" => Some(EMFILE),
            "EFAULT" => Some(EFAULT),
            "EACCESS" => Some(EACCESS),
            "ENETRESET" => Some(ENETRESET),
            "ENETUNREACH" => Some(ENETUNREACH),
            "EHOSTUNREACH" => Some(EHOSTUNREACH),
            "ENOTCONN" => Some(ENOTCONN),
            "EMSGSIZE" => Some(EMSGSIZE),
            "ETIMEDOUT" => Some(ETIMEDOUT),
            "ECONNABORTED" => Some(ECONNABORTED),
            "ECONNRESET" => Some(ECONNRESET),
            "ENOPROTOOPT" => Some(ENOPROTOOPT),
            "EISCONN" => Some(EISCONN),
            "ESOCKTNOSUPPORT" => Some(ESOCKTNOSUPPORT),
            _ => None
        }
    }
}
