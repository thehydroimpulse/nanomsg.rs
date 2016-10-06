extern crate nanomsg_sys;
extern crate libc;

pub use result::{Result, Error};
pub use endpoint::Endpoint;

use nanomsg_sys::nn_pollfd;

use libc::{c_int, c_void, size_t};
use std::ffi::CString;
use std::cmp;
use std::mem;
use std::str;
use std::ptr;
use result::last_nano_error;
use std::io;
use std::mem::size_of;
use std::slice;
use std::convert::From;

#[cfg(unix)]
use std::os::unix::io::RawFd;

#[cfg(windows)]
use std::os::windows::raw::SOCKET;

pub mod result;
pub mod endpoint;

/// Type-safe protocols that Nanomsg uses. Each socket
/// is bound to a single protocol that has specific behaviour
/// (such as only being able to receive messages and not send them).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Protocol {
    /// Used to implement the client application that sends requests and receives replies.
    ///
    /// **See also:** `Socket::set_request_resend_interval`
    Req = (nanomsg_sys::NN_REQ) as isize,

    /// Used to implement the stateless worker that receives requests and sends replies.
    Rep = (nanomsg_sys::NN_REP) as isize,

    /// This socket is used to send messages to a cluster of load-balanced nodes.
    /// Receive operation is not implemented on this socket type.
    Push = (nanomsg_sys::NN_PUSH) as isize,

    /// This socket is used to receive a message from a cluster of nodes.
    /// Send operation is not implemented on this socket type.
    Pull = (nanomsg_sys::NN_PULL) as isize,

    /// Socket for communication with exactly one peer.
    /// Each party can send messages at any time.
    /// If the peer is not available or send buffer is full subsequent calls to `write`
    /// will block until it’s possible to send the message.
    Pair = (nanomsg_sys::NN_PAIR) as isize,

    /// Sent messages are distributed to all nodes in the topology.
    /// Incoming messages from all other nodes in the topology are fair-queued in the socket.
    Bus = (nanomsg_sys::NN_BUS) as isize,

    /// This socket is used to distribute messages to multiple destinations.
    /// Receive operation is not defined.
    Pub = (nanomsg_sys::NN_PUB) as isize,

    /// Receives messages from the publisher.
    /// Only messages that the socket is subscribed to are received.
    /// When the socket is created there are no subscriptions and thus no messages will be received.
    /// Send operation is not defined on this socket.
    ///
    /// **See also:** `Socket::subscribe` and `Socket::unsubscribe`.
    Sub = (nanomsg_sys::NN_SUB) as isize,

    /// Used to send the survey.
    /// The survey is delivered to all the connected respondents.
    /// Once the query is sent, the socket can be used to receive the responses.
    /// When the survey deadline expires, receive will return Timeout error.
    ///
    /// **See also:** `Socket::set_survey_deadline`
    Surveyor = (nanomsg_sys::NN_SURVEYOR) as isize,

    /// Use to respond to the survey.
    /// Survey is received using receive function, response is sent using send function
    /// This socket can be connected to at most one peer.
    Respondent = (nanomsg_sys::NN_RESPONDENT) as isize
}

impl Protocol {
    fn to_raw(&self) -> c_int {
        *self as c_int
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Transport {
    /// In-process transport
    Inproc = (nanomsg_sys::NN_INPROC) as isize,
    /// Inter-process transport
    Ipc = (nanomsg_sys::NN_IPC) as isize,
    /// TCP transport
    Tcp = (nanomsg_sys::NN_TCP) as isize
}

impl Transport {
    pub fn to_raw(&self) -> c_int {
        *self as c_int
    }
}

/// A type-safe socket wrapper around nanomsg's own socket implementation. This
/// provides a safe interface for dealing with initializing the sockets, sending
/// and receiving messages.
pub struct Socket {
    socket: c_int
}

#[derive(Clone, Copy)]
pub enum PollInOut {
    /// Check whether at least one message can be received from the socket without blocking.
    In,
    /// Check whether at least one message can be sent to the fd socket without blocking.
    Out,
    /// Check whether at least one message can be sent to or received from the fd socket without blocking.
    InOut,
}

#[derive(Clone, Copy)]
/// A request for polling a socket and the poll result.
/// To create the request, see `Socket::new_pollfd`.
/// To get the result, see `PollFd::can_read` and `PollFd::can_write`.
pub struct PollFd {
    socket: c_int,
    check_pollinout: PollInOut,
    check_pollin_result: bool,
    check_pollout_result: bool
}

impl PollFd {

    fn convert(&self) -> nn_pollfd {
        let (pollin, pollout) = match self.check_pollinout {
            PollInOut::In    => (true,  false),
            PollInOut::Out   => (false, true),
            PollInOut::InOut => (true,  true),
        };
        nn_pollfd::new(self.socket, pollin, pollout)
    }

    /// Checks whether at least one message can be received from the socket without blocking.
    pub fn can_read(&self) -> bool {
        self.check_pollin_result
    }

    /// Checks whether at least one message can be sent to the fd socket without blocking.
    pub fn can_write(&self) -> bool {
        self.check_pollout_result
    }

}

/// A request for polling a set of sockets and the poll results.
/// To create the request, see `PollRequest::new`.
pub struct PollRequest<'a> {
    fds: &'a mut [PollFd],
    nn_fds: Vec<nn_pollfd>
}

impl<'a> PollRequest<'a> {
    /// Creates a request from the specified individualsocket requests.
    pub fn new(fds: &'a mut [PollFd]) -> PollRequest<'a> {
        let nn_fds = fds.iter().map(|fd| fd.convert()).collect();

        PollRequest { fds: fds, nn_fds: nn_fds }
    }

    fn len(&self) -> usize {
        self.fds.len()
    }

    /// Returns a reference to the socket requests, so they can be checked.
    pub fn get_fds(&'a self) -> &'a [PollFd] {
        self.fds
    }

    fn get_nn_fds(&mut self) -> *mut nn_pollfd {
        self.nn_fds.as_mut_ptr()
    }

    fn copy_poll_result(&mut self) {

        for x in 0..self.fds.len() {
            self.fds[x].check_pollin_result = self.nn_fds[x].pollin_result();
            self.fds[x].check_pollout_result = self.nn_fds[x].pollout_result();
        }
    }
}

macro_rules! error_guard(
    ($ret:ident) => (
        if $ret == -1 {
            return Err(last_nano_error())
        }
    )
);

macro_rules! io_error_guard(
    ($ret:ident) => (
        if $ret == -1 {
            let nano_err = last_nano_error();
            let io_err = From::from(nano_err);
            return Err(io_err);
        }
    )
);

impl Socket {

    /// Allocate and initialize a new Nanomsg socket which returns
    /// a new file descriptor behind the scene. The safe interface doesn't
    /// expose any of the underlying file descriptors and such.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol};
    ///
    /// let mut socket = match Socket::new(Protocol::Pull) {
    ///     Ok(socket) => socket,
    ///     Err(err) => panic!("{}", err)
    /// };
    /// ```
    ///
    /// # Error
    ///
    /// - `AddressFamilyNotSupported` : Specified address family is not supported.
    /// - `InvalidArgument` : Unknown protocol.
    /// - `TooManyOpenFiles` : The limit on the total number of open SP sockets or OS limit for file descriptors has been reached.
    /// - `Terminating` : The library is terminating.
    pub fn new(protocol: Protocol) -> Result<Socket> {
        Socket::create_socket(nanomsg_sys::AF_SP, protocol)
    }

    /// Allocate and initialize a new Nanomsg socket meant to be used in a device
    ///
    /// # Example
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol};
    ///
    /// let mut s1 = Socket::new_for_device(Protocol::Req).unwrap();
    /// let mut s2 = Socket::new_for_device(Protocol::Rep).unwrap();
    /// let ep1 = s1.bind("ipc:///tmp/new_for_device1.ipc").unwrap();
    /// let ep2 = s2.bind("ipc:///tmp/new_for_device2.ipc").unwrap();
    ///
    /// // And now `Socket::device(&s1, &s2)` can be called to create the device.
    /// ```
    pub fn new_for_device(protocol: Protocol) -> Result<Socket> {
        Socket::create_socket(nanomsg_sys::AF_SP_RAW, protocol)
    }

    fn create_socket(domain: c_int, protocol: Protocol) -> Result<Socket> {
        let socket = unsafe { nanomsg_sys::nn_socket(domain, protocol.to_raw()) };

        error_guard!(socket);
        Ok(Socket {socket: socket})
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
    /// # Example
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol};
    ///
    /// let mut socket = match Socket::new(Protocol::Push) {
    ///     Ok(socket) => socket,
    ///     Err(err) => panic!("{}", err)
    /// };
    ///
    /// // Bind the newly created socket to the following address:
    /// match socket.bind("ipc:///tmp/bind_doc.ipc") {
    ///     Ok(_) => {},
    ///     Err(err) => panic!("Failed to bind socket: {}", err)
    /// }
    /// ```
    ///
    /// # Error
    ///
    /// - `BadFileDescriptor` : The socket is invalid.
    /// - `TooManyOpenFiles` : Maximum number of active endpoints was reached.
    /// - `InvalidArgument` : The syntax of the supplied address is invalid.
    /// - `NameTooLong` : The supplied address is too long.
    /// - `ProtocolNotSupported` : The requested transport protocol is not supported.
    /// - `AddressNotAvailable` : The requested endpoint is not local.
    /// - `NoDevice` : Address specifies a nonexistent interface.
    /// - `AddressInUse` : The requested local endpoint is already in use.
    /// - `Terminating` : The library is terminating.
    pub fn bind(&mut self, addr: &str) -> Result<Endpoint> {
        let c_addr = CString::new(addr.as_bytes());
        if c_addr.is_err() {
            return Err(Error::from_raw(nanomsg_sys::EINVAL));
        }
        let ret = unsafe { nanomsg_sys::nn_bind(self.socket, c_addr.unwrap().as_ptr()) };

        error_guard!(ret);
        Ok(Endpoint::new(ret, self.socket))
    }

    /// Connects the socket to a remote endpoint.
    /// Returns the endpoint on success.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol};
    ///
    /// let mut socket = match Socket::new(Protocol::Pull) {
    ///     Ok(socket) => socket,
    ///     Err(err) => panic!("{}", err)
    /// };
    ///
    /// let endpoint = match socket.connect("ipc:///tmp/connect_doc.ipc") {
    ///     Ok(ep) => ep,
    ///     Err(err) => panic!("Failed to connect socket: {}", err)
    /// };
    /// ```
    ///
    /// # Error
    ///
    /// - `BadFileDescriptor` : The socket is invalid.
    /// - `TooManyOpenFiles` : Maximum number of active endpoints was reached.
    /// - `InvalidArgument` : The syntax of the supplied address is invalid.
    /// - `NameTooLong` : The supplied address is too long.
    /// - `ProtocolNotSupported` : The requested transport protocol is not supported.
    /// - `NoDevice` : Address specifies a nonexistent interface.
    /// - `Terminating` : The library is terminating.
    pub fn connect(&mut self, addr: &str) -> Result<Endpoint> {
        let c_addr = CString::new(addr.as_bytes());
        if c_addr.is_err() {
            return Err(Error::from_raw(nanomsg_sys::EINVAL));
        }
        let ret = unsafe { nanomsg_sys::nn_connect(self.socket, c_addr.unwrap().as_ptr()) };

        error_guard!(ret);
        Ok(Endpoint::new(ret, self.socket))
    }

    /// Non-blocking version of the `read` function.
    /// Any bytes exceeding the length specified by `buf.len()` will be truncated.
    /// Returns the number of bytes of the message stored in the buffer on success.
    /// Please note that it differs from nanomsg's nn_recv which returns the msg size instead.
    /// An error with the `Error::TryAgain` kind is returned if there's no message to receive for the moment.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol, Error};
    ///
    /// let mut socket = Socket::new(Protocol::Pull).unwrap();
    /// let mut endpoint = socket.connect("ipc:///tmp/nb_read_doc.ipc").unwrap();
    /// let mut buffer = [0u8; 1024];
    ///
    /// match socket.nb_read(&mut buffer) {
    ///     Ok(count) => {
    ///         println!("Read {} bytes !", count);
    ///         // here we can process the message stored in `buffer`
    ///     },
    ///     Err(Error::TryAgain) => {
    ///         println!("Nothing to be read for the moment ...");
    ///         // here we can use the CPU for something else and try again later
    ///     },
    ///     Err(err) => panic!("Problem while reading: {}", err)
    /// };
    /// ```
    ///
    /// # Error
    ///
    /// - `BadFileDescriptor` : The socket is invalid.
    /// - `OperationNotSupported` : The operation is not supported by this socket type.
    /// - `FileStateMismatch` : The operation cannot be performed on this socket at the moment because socket is not in the appropriate state. This error may occur with socket types that switch between several states.
    /// - `TryAgain` : Non-blocking mode was requested and there’s no message to receive at the moment.
    /// - `Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `Terminating` : The library is terminating.
    pub fn nb_read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let buf_len = buf.len();
        let buf_ptr = buf.as_mut_ptr();
        let c_buf_len = buf_len as size_t;
        let c_buf_ptr = buf_ptr as *mut c_void;
        let ret = unsafe {
            nanomsg_sys::nn_recv(self.socket, c_buf_ptr, c_buf_len, nanomsg_sys::NN_DONTWAIT)
        };

        error_guard!(ret);
        Ok(cmp::min(ret as usize, buf_len))
    }

    /// Non-blocking version of the `read_to_end` function.
    /// Copy the message allocated by nanomsg into the buffer on success.
    /// An error with the `Error::TryAgain` kind is returned if there's no message to receive for the moment.
    ///
    /// # Example:
    ///
    /// ```rust
    /// #![allow(unstable)]
    /// use nanomsg::{Socket, Protocol, Error};
    ///
    /// let mut socket = Socket::new(Protocol::Pull).unwrap();
    /// let mut endpoint = socket.connect("ipc:///tmp/nb_read_to_end_doc.ipc").unwrap();
    ///
    /// let mut buffer = Vec::new();
    /// match socket.nb_read_to_end(&mut buffer) {
    ///     Ok(_) => {
    ///         println!("Read message {} bytes !", buffer.len());
    ///         // here we can process the message stored in `buffer`
    ///     },
    ///     Err(Error::TryAgain) => {
    ///         println!("Nothing to be read for the moment ...");
    ///         // here we can use the CPU for something else and try again later
    ///     },
    ///     Err(err) => panic!("Problem while reading: {}", err)
    /// };
    /// ```
    ///
    /// # Error
    ///
    /// - `BadFileDescriptor` : The socket is invalid.
    /// - `OperationNotSupported` : The operation is not supported by this socket type.
    /// - `FileStateMismatch` : The operation cannot be performed on this socket at the moment because socket is not in the appropriate state. This error may occur with socket types that switch between several states.
    /// - `TryAgain` : Non-blocking mode was requested and there’s no message to receive at the moment.
    /// - `Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `Terminating` : The library is terminating.
    pub fn nb_read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let mut msg : *mut u8 = ptr::null_mut();
        let ret = unsafe {
            nanomsg_sys::nn_recv(self.socket, mem::transmute(&mut msg), nanomsg_sys::NN_MSG, nanomsg_sys::NN_DONTWAIT)
        };

        error_guard!(ret);

        let ret = ret as usize;
        let bytes = unsafe { slice::from_raw_parts(msg, ret) };
        buf.extend_from_slice(bytes);
        unsafe { nanomsg_sys::nn_freemsg(msg as *mut c_void) };
        Ok(ret)
    }

    /// Non-blocking version of the `write` function.
    /// An error with the `Error::TryAgain` kind is returned if the message cannot be sent at the moment.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol, Error};
    ///
    /// let mut socket = Socket::new(Protocol::Push).unwrap();
    /// let mut endpoint = socket.connect("ipc:///tmp/nb_write_doc.ipc").unwrap();
    ///
    /// match socket.nb_write(b"foobar") {
    ///     Ok(_) => { println!("Message sent !"); },
    ///     Err(Error::TryAgain) => {
    ///         println!("Receiver not ready, message can't be sent for the moment ...");
    ///     },
    ///     Err(err) => panic!("Problem while writing: {}", err)
    /// };
    /// ```
    ///
    /// # Error
    ///
    /// - `BadFileDescriptor` : The socket is invalid.
    /// - `OperationNotSupported` : The operation is not supported by this socket type.
    /// - `FileStateMismatch` : The operation cannot be performed on this socket at the moment because socket is not in the appropriate state. This error may occur with socket types that switch between several states.
    /// - `TryAgain` : Non-blocking mode was requested and there’s no message to receive at the moment.
    /// - `Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `Terminating` : The library is terminating.
    pub fn nb_write(&mut self, buf: &[u8]) -> Result<usize> {
        let buf_ptr = buf.as_ptr() as *const c_void;
        let buf_len = buf.len() as size_t;
        let ret = unsafe {
            nanomsg_sys::nn_send(self.socket, buf_ptr, buf_len, nanomsg_sys::NN_DONTWAIT)
        };

        error_guard!(ret);
        Ok(buf_len as usize)
    }

    /// Zero-copy version of the `write` function.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol};
    /// use std::io::{Read, Write};
    ///
    /// let mut push_socket = Socket::new(Protocol::Push).unwrap();
    /// let mut push_endpoint = push_socket.bind("ipc:///tmp/zc_write_doc.ipc").unwrap();
    /// let mut pull_socket = Socket::new(Protocol::Pull).unwrap();
    /// let mut pull_endpoint = pull_socket.connect("ipc:///tmp/zc_write_doc.ipc").unwrap();
    /// let mut msg = Socket::allocate_msg(6).unwrap();
    /// msg[0] = 102u8;
    /// msg[1] = 111u8;
    /// msg[2] = 111u8;
    /// msg[3] = 98u8;
    /// msg[4] = 97u8;
    /// msg[5] = 114u8;
    ///
    /// match push_socket.zc_write(msg) {
    ///     Ok(_) => { println!("Message sent, do not try to reuse it !"); },
    ///     Err(err) => panic!("Problem while writing: {}, msg still available", err)
    /// };
    /// let mut text = String::new();
    /// match pull_socket.read_to_string(&mut text) {
    ///     Ok(_) => { println!("Message received."); },
    ///     Err(err) => panic!("{}", err)
    /// }
    /// ```
    ///
    /// # Error
    ///
    /// - `BadFileDescriptor` : The socket is invalid.
    /// - `OperationNotSupported` : The operation is not supported by this socket type.
    /// - `FileStateMismatch` : The operation cannot be performed on this socket at the moment because socket is not in the appropriate state. This error may occur with socket types that switch between several states.
    /// - `Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `Terminating` : The library is terminating.
    pub fn zc_write(&mut self, buf: &[u8]) -> Result<usize> {
        let ptr = buf.as_ptr() as *const c_void;
        let ptr_addr = &ptr as *const _ as *const c_void;
        let len = buf.len();
        let ret = unsafe { nanomsg_sys::nn_send(self.socket, ptr_addr, nanomsg_sys::NN_MSG, 0) };

        error_guard!(ret);
        Ok(len)
    }

    /// Allocate a message of the specified size to be sent in zero-copy fashion.
    /// The content of the message is undefined after allocation and it should be filled in by the user.
    /// While `write` functions allow to send arbitrary buffers,
    /// buffers allocated using `allocate_msg` can be more efficient for large messages
    /// as they allow for using zero-copy techniques.
    ///
    /// # Error
    ///
    /// - `InvalidArgument` : Supplied allocation type is invalid.
    /// - `Unknown` : Out of memory.
    pub fn allocate_msg<'a>(len: usize) -> Result<&'a mut [u8]> {
        unsafe {
            let ptr = nanomsg_sys::nn_allocmsg(len as size_t, 0) as *mut u8;
            let ptr_value = ptr as isize;

            if ptr_value == 0 {
                return Err(last_nano_error());
            }

            Ok(slice::from_raw_parts_mut(ptr, len))
        }
    }

    /// Deallocates a message allocated using `allocate_msg` function
    ///
    /// # Error
    ///
    /// - `BadAddress` : The message pointer is invalid.
    pub fn free_msg<'a>(msg: &'a mut [u8]) -> Result<()> {
        unsafe {
            let ptr = msg.as_mut_ptr() as *mut c_void;
            let ret = nanomsg_sys::nn_freemsg(ptr);

            error_guard!(ret);
            Ok(())
        }
    }

    /// Creates a poll request for the socket with the specified check criteria.
    /// - **pollinout:** See `PollInOut` for options
    pub fn new_pollfd(&self, pollinout: PollInOut) -> PollFd {
        PollFd {
            socket: self.socket,
            check_pollinout: pollinout,
            check_pollin_result: false,
            check_pollout_result: false
        }
    }

    /// Checks a set of sockets and reports whether it’s possible to send a message to the socket and/or receive a message from each socket.
    /// Upon successful completion, the number of `PollFd` structures with events signaled is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol, PollFd, PollRequest, PollInOut};
    /// use std::thread;
    ///
    /// let mut left_socket = Socket::new(Protocol::Pair).unwrap();
    /// let mut left_ep = left_socket.bind("ipc:///tmp/poll_doc.ipc").unwrap();
    ///
    /// let mut right_socket = Socket::new(Protocol::Pair).unwrap();
    /// let mut right_ep = right_socket.connect("ipc:///tmp/poll_doc.ipc").unwrap();
    ///
    /// thread::sleep_ms(10);
    ///
    /// // Here some messages may have been sent ...
    ///
    /// let mut pollfd_vec: Vec<PollFd> = vec![left_socket.new_pollfd(PollInOut::InOut), right_socket.new_pollfd(PollInOut::InOut)];
    /// let mut poll_req = PollRequest::new(&mut pollfd_vec[..]);
    /// let timeout = 10;
    /// let poll_result = Socket::poll(&mut poll_req, timeout);
    ///
    /// if poll_req.get_fds()[0].can_write() {
    ///     // left_socket socket can send a message ...
    /// }
    ///
    /// if poll_req.get_fds()[1].can_read() {
    ///     // right_socket socket is ready to receive a message ...
    /// }
    /// ```
    ///
    /// # Error
    ///
    /// - `BadFileDescriptor` : Some of the provided sockets are invalid.
    /// - `Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `Timeout` : No event was signaled before the specified timeout.
    /// - `Terminating` : The library is terminating.
    pub fn poll(request: &mut PollRequest, timeout: isize) -> Result<usize> {
        let nn_fds = request.get_nn_fds();
        let len = request.len() as c_int;
        let ret = unsafe { nanomsg_sys::nn_poll(nn_fds, len, timeout as c_int) };

        error_guard!(ret);

        if ret == 0 {
            return Err(Error::TimedOut);
        }

        request.copy_poll_result();

        Ok(ret as usize)
    }

    /// Starts a device to forward messages between two sockets.
    /// If both sockets are valid, `device` function loops
    /// and sends and messages received from s1 to s2 and vice versa.
    /// If only one socket is valid and the other is negative,
    /// `device` works in a "loopback" mode —
    /// it loops and sends any messages received from the socket back to itself.
    /// To break the loop and make `device` function exit use `terminate` function.
    ///
    /// # Error
    ///
    /// - `BadFileDescriptor` : Some of the provided sockets are invalid.
    /// - `Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `InvalidArgument` : Either one of the socket is not an AF_SP_RAW socket; or the two sockets don’t belong to the same protocol; or the directionality of the sockets doesn’t fit (e.g. attempt to join two SINK sockets to form a device).
    /// - `Terminating` : The library is terminating.
    pub fn device(socket1: &Socket, socket2: &Socket) -> Result<()> {
        let ret = unsafe { nanomsg_sys::nn_device(socket1.socket, socket2.socket) };

        error_guard!(ret);
        Ok(())
    }

    /// Notify all sockets about process termination.
    /// To help with shutdown of multi-threaded programs nanomsg provides the `terminate` function
    /// which informs all the open sockets that process termination is underway.
    /// If a socket is blocked inside a blocking function, such as `read`,
    /// it will be unblocked and `Terminating` error will be returned to the user.
    /// Similarly, any subsequent attempt to invoke a socket function other than `drop` after `terminate` was called will result in `Terminating` error.
    /// If waiting inside a polling function, the call will unblock with both read and write signaled.
    /// The `terminate` function itself is non-blocking.
    pub fn terminate() {
        unsafe { nanomsg_sys::nn_term() };
    }

    fn set_socket_options_c_int(&self, level: c_int, option: c_int, val: c_int) -> Result<()> {
        let val_ptr = &val as *const _ as *const c_void;

        let ret = unsafe {
            nanomsg_sys::nn_setsockopt(self.socket,
                                      level,
                                      option,
                                      val_ptr,
                                      size_of::<c_int>() as size_t)
        };

        error_guard!(ret);
        Ok(())
    }

    fn set_socket_options_str(&self, level: c_int, option: c_int, val: &str) -> Result<()> {
        let c_val = CString::new(val.as_bytes());
        if c_val.is_err() {
            return Err(Error::from_raw(nanomsg_sys::EINVAL));
        }
        let ptr = c_val.unwrap().as_ptr() as *const c_void;
        let ret = unsafe {
            nanomsg_sys::nn_setsockopt(self.socket,
                                      level,
                                      option,
                                      ptr,
                                      val.len() as size_t)
        };

        error_guard!(ret);
        Ok(())
    }

    fn get_socket_option_c_int(&self, level: c_int, option: c_int) -> Result<c_int> {
        let mut val: c_int = 0;
        let mut sz: size_t = size_of::<c_int>();
        let val_ptr = &mut val as *mut _ as *mut c_void;
        let sz_ptr = &mut sz as *mut size_t;

        let ret = unsafe {
            nanomsg_sys::nn_getsockopt(self.socket,
                                      level,
                                      option,
                                      val_ptr,
                                      sz_ptr)
        };
        error_guard!(ret);
        Ok(val)
    }

    fn get_socket_option_str(&self, level: c_int, option: c_int, len: size_t) -> Result<CString> {
        let val: Vec<u8> = Vec::with_capacity(len);
        let mut sz: size_t = len; // Copy len so that we don't mutate paramater

        let c_val = CString::new(val);
        if c_val.is_err() {
            return Err(Error::from_raw(nanomsg_sys::EINVAL));
        }

        let val_ptr = c_val.unwrap().into_raw();
        let sz_ptr = &mut sz as *mut size_t;

        let ret = unsafe {
            nanomsg_sys::nn_getsockopt(self.socket,
                                       level,
                                       option,
                                       val_ptr as *mut c_void,
                                       sz_ptr)
        };
        error_guard!(ret);
        unsafe {
            Ok(CString::from_raw(val_ptr))
        }
    }

    /// Specifies how long the socket should try to send pending outbound messages after `drop` have been called.
    /// Negative value means infinite linger. Default value is 1000 (1 second).
    pub fn set_linger(&mut self, linger: isize) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_LINGER,
                                      linger as c_int)
    }

    /// Size of the send buffer, in bytes. To prevent blocking for messages larger than the buffer,
    /// exactly one message may be buffered in addition to the data in the send buffer.
    /// Default value is 128kB.
    pub fn set_send_buffer_size(&mut self, size_in_bytes: usize) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_SNDBUF,
                                      size_in_bytes as c_int)
    }

    /// Size of the receive buffer, in bytes. To prevent blocking for messages larger than the buffer,
    /// exactly one message may be buffered in addition to the data in the receive buffer.
    /// Default value is 128kB.
    pub fn set_receive_buffer_size(&mut self, size_in_bytes: usize) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_RCVBUF,
                                      size_in_bytes as c_int)
    }

    /// The timeout for send operation on the socket.
    /// If message cannot be sent within the specified timeout, TryAgain error is returned.
    /// Negative value means infinite timeout. Default value is infinite timeout.
    pub fn set_send_timeout(&mut self, timeout: isize) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_SNDTIMEO,
                                      timeout as c_int)
    }

    /// The timeout for recv operation on the socket.
    /// If message cannot be received within the specified timeout, TryAgain error is returned.
    /// Negative value means infinite timeout. Default value is infinite timeout.
    pub fn set_receive_timeout(&mut self, timeout: isize) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_RCVTIMEO,
                                      timeout as c_int)
    }

    /// For connection-based transports such as TCP, this option specifies how long to wait,
    /// when connection is broken before trying to re-establish it.
    /// Note that actual reconnect interval may be randomised to some extent to prevent severe reconnection storms.
    /// Default value is 100 milliseconds.
    pub fn set_reconnect_interval(&mut self, interval: isize) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_RECONNECT_IVL,
                                      interval as c_int)
    }

    /// This option is to be used only in addition to `set_reconnect_interval` option.
    /// It specifies maximum reconnection interval. On each reconnect attempt,
    /// the previous interval is doubled until `max_reconnect_interval` is reached.
    /// Value of zero means that no exponential backoff is performed and
    /// reconnect interval is based only on `reconnect_interval`.
    /// If `max_reconnect_interval` is less than `reconnect_interval`, it is ignored.
    /// Default value is 0.
    pub fn set_max_reconnect_interval(&mut self, interval: isize) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_RECONNECT_IVL_MAX,
                                      interval as c_int)
    }

    /// Sets outbound priority for endpoints subsequently added to the socket.
    /// This option has no effect on socket types that send messages to all the peers.
    /// However, if the socket type sends each message to a single peer (or a limited set of peers),
    /// peers with high priority take precedence over peers with low priority.
    /// Highest priority is 1, lowest priority is 16. Default value is 8.
    pub fn set_send_priority(&mut self, priority: u8) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_SNDPRIO,
                                      priority as c_int)
    }

    /// Sets inbound priority for endpoints subsequently added to the socket.
    /// This option has no effect on socket types that are not able to receive messages.
    /// When receiving a message, messages from peer with higher priority are received before messages
    /// from peer with lower priority.
    /// Highest priority is 1, lowest priority is 16. Default value is 8.
    pub fn set_receive_priority(&mut self, priority: u8) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_RCVPRIO,
                                      priority as c_int)
    }

    /// If set to true, only IPv4 addresses are used.
    /// If set to false, both IPv4 and IPv6 addresses are used.
    /// Default value is true.
    pub fn set_ipv4_only(&mut self, ipv4_only: bool) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SOL_SOCKET,
                                      nanomsg_sys::NN_IPV4ONLY,
                                      ipv4_only as c_int)
    }

    /// Socket name for error reporting and statistics.
    /// Default value is "socket.N" where N is socket integer.
    /// **This option is experimental, see `Socket::env` for details**
    #[cfg(not(windows))]
    pub fn set_socket_name(&mut self, name: &str) -> Result<()> {
        self.set_socket_options_str(nanomsg_sys::NN_SOL_SOCKET,
                                    nanomsg_sys::NN_SOCKET_NAME,
                                    name)
    }

    /// This option, when set to `true`, disables Nagle’s algorithm.
    /// It also disables delaying of TCP acknowledgments.
    /// Using this option improves latency at the expense of throughput.
    pub fn set_tcp_nodelay(&mut self, tcp_nodelay: bool) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_TCP,
                                      nanomsg_sys::NN_TCP_NODELAY,
                                      tcp_nodelay as c_int)
    }

    /// Retrieve a file descriptor that is readable when a message can
	/// be received on the unerlying socket
	#[cfg(unix)]
	pub fn get_receive_fd(&mut self) -> Result<RawFd> {
		self.get_socket_option_c_int(nanomsg_sys::NN_SOL_SOCKET,
									 nanomsg_sys::NN_RCVFD).map(|v: c_int| {
										 v as RawFd
									 })
	}

	#[cfg(windows)]
	pub fn get_receive_fd(&mut self) -> Result<SOCKET> {
		self.get_socket_option_c_int(nanomsg_sys::NN_SOL_SOCKET,
									 nanomsg_sys::NN_RCVFD).map(|v: c_int| {
										 v as SOCKET
									 })
	}

    /// Retrieve a file descriptor that is writeable when a message
	/// can be sent on the underlying socket
	#[cfg(unix)]
	pub fn get_send_fd(&mut self) -> Result<RawFd> {
		self.get_socket_option_c_int(nanomsg_sys::NN_SOL_SOCKET,
									 nanomsg_sys::NN_SNDFD).map(|v: c_int| {
										 v as RawFd
									 })
	}

	#[cfg(windows)]
	pub fn get_send_fd(&mut self) -> Result<SOCKET> {
		self.get_socket_option_c_int(nanomsg_sys::NN_SOL_SOCKET,
									 nanomsg_sys::NN_SNDFD).map(|v: c_int| {
										 v as SOCKET
									 })
	}

    /// Retrieve the name for this socket for error reporting and
    /// statistics.
    /// **This option is experimental, see `Socket::env` for details
    #[cfg(not(windows))]
    pub fn get_socket_name(&mut self, len: usize) -> Result<String> {
        self.get_socket_option_str(nanomsg_sys::NN_SOL_SOCKET,
                                   nanomsg_sys::NN_SOCKET_NAME,
                                   len).map(|v: CString| {
                                       v.to_string_lossy().into_owned()
                                   })
    }

    /// Defined on full `Sub` socket.
    /// Subscribes for a particular topic.
    /// Type of the option is string.
    /// A single `Sub` socket can handle multiple subscriptions.
    pub fn subscribe(&mut self, topic: &str) -> Result<()> {
        self.set_socket_options_str(nanomsg_sys::NN_SUB,
                                    nanomsg_sys::NN_SUB_SUBSCRIBE,
                                    topic)
    }

    /// Defined on full `Sub` socket. Unsubscribes from a particular topic.
    pub fn unsubscribe(&mut self, topic: &str) -> Result<()> {
        self.set_socket_options_str(nanomsg_sys::NN_SUB,
                                    nanomsg_sys::NN_SUB_UNSUBSCRIBE,
                                    topic)
    }

    /// Specifies how long to wait for responses to the survey.
    /// Once the deadline expires, receive function will return `Timeout` error and all subsequent responses to the survey will be silently dropped.
    /// The deadline is measured in milliseconds. Default value is 1 second.
    pub fn set_survey_deadline(&mut self, deadline: isize) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_SURVEYOR,
                                      nanomsg_sys::NN_SURVEYOR_DEADLINE,
                                      deadline as c_int)
    }

    /// This option is defined on the full `Req` socket.
    /// If reply is not received in specified amount of milliseconds, the request will be automatically resent.
    /// The type of this option is int. Default value is 1 minute.
    pub fn set_request_resend_interval(&mut self, interval: isize) -> Result<()> {
        self.set_socket_options_c_int(nanomsg_sys::NN_REQ,
                                      nanomsg_sys::NN_REQ_RESEND_IVL,
                                      interval as c_int)
    }

}

impl io::Read for Socket {
    /// Receive a message from the socket and store it in the buffer argument.
    /// Any bytes exceeding the length specified by `buffer.len()` will be truncated.
    /// Returns the number of bytes of the message stored in the buffer on success.
    /// Please note that it differs from nanomsg's nn_recv which returns the msg size instead.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol};
    /// use std::thread;
    /// use std::io::{Read, Write};
    ///
    /// let mut push_socket = Socket::new(Protocol::Push).unwrap();
    /// let mut push_ep = push_socket.bind("ipc:///tmp/read_doc.ipc").unwrap();
    ///
    /// let mut pull_socket = Socket::new(Protocol::Pull).unwrap();
    /// let mut pull_ep = pull_socket.connect("ipc:///tmp/read_doc.ipc").unwrap();
    /// let mut buffer = [0u8; 1024];
    ///
    /// thread::sleep_ms(50);
    ///
    /// match push_socket.write(b"foobar") {
    ///     Ok(..) => println!("Message sent !"),
    ///     Err(err) => panic!("Failed to write to the socket: {}", err)
    /// }
    ///
    /// match pull_socket.read(&mut buffer) {
    ///     Ok(count) => {
    ///         println!("Read {} bytes !", count);
    ///         // here we can process the `count` bytes of the message stored in `buffer`
    ///     },
    ///     Err(err) => panic!("Problem while reading: {}", err)
    /// };
    /// ```
    ///
    /// # Error
    ///
    /// - `io::ErrorKind::FileNotFound` : The socket is invalid.
    /// - `io::ErrorKind::MismatchedFileTypeForOperation` : The operation is not supported by this socket type.
    /// - `io::ErrorKind::ResourceUnavailable` : The operation cannot be performed on this socket at the moment because socket is not in the appropriate state. This error may occur with socket types that switch between several states.
    /// - `io::ErrorKind::TimedOut` : Individual socket types may define their own specific timeouts. If such timeout is hit this error will be returned.
    /// - `io::ErrorKind::Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `io::ErrorKind::Other` : The library is terminating.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let buf_len = buf.len();
        let buf_ptr = buf.as_mut_ptr();
        let c_buf_len = buf_len as size_t;
        let c_buf_ptr = buf_ptr as *mut c_void;

        let ret = unsafe { nanomsg_sys::nn_recv(self.socket, c_buf_ptr, c_buf_len, 0) };

        io_error_guard!(ret);
        Ok(cmp::min(ret as usize, buf_len))
    }

    /// Receive a message from the socket. Copy the message allocated by nanomsg into the buffer on success.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol};
    /// use std::thread;
    /// use std::io::{Read, Write};
    ///
    /// let mut push_socket = Socket::new(Protocol::Push).unwrap();
    /// let mut push_ep = push_socket.bind("ipc:///tmp/read_to_end_doc.ipc").unwrap();
    ///
    /// let mut pull_socket = Socket::new(Protocol::Pull).unwrap();
    /// let mut pull_ep = pull_socket.connect("ipc:///tmp/read_to_end_doc.ipc").unwrap();
    ///
    /// thread::sleep_ms(50);
    ///
    /// match push_socket.write(b"foobar") {
    ///     Ok(..) => println!("Message sent !"),
    ///     Err(err) => panic!("Failed to write to the socket: {}", err)
    /// }
    ///
    /// let mut msg = Vec::new();
    /// match pull_socket.read_to_end(&mut msg) {
    ///     Ok(_) => {
    ///         println!("Read {} bytes !", msg.len());
    ///         // here we can process the the message stored in `msg`
    ///     },
    ///     Err(err) => panic!("Problem while reading: {}", err)
    /// };
    /// ```
    ///
    /// # Error
    ///
    /// - `io::ErrorKind::FileNotFound` : The socket is invalid.
    /// - `io::ErrorKind::MismatchedFileTypeForOperation` : The operation is not supported by this socket type.
    /// - `io::ErrorKind::ResourceUnavailable` : The operation cannot be performed on this socket at the moment because socket is not in the appropriate state. This error may occur with socket types that switch between several states.
    /// - `io::ErrorKind::TimedOut` : Individual socket types may define their own specific timeouts. If such timeout is hit this error will be returned.
    /// - `io::ErrorKind::Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `io::ErrorKind::Other` : The library is terminating.
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut msg : *mut u8 = ptr::null_mut();
        let ret = unsafe { nanomsg_sys::nn_recv(self.socket, mem::transmute(&mut msg), nanomsg_sys::NN_MSG, 0) };

        io_error_guard!(ret);

        let ret = ret as usize;
        let bytes = unsafe { slice::from_raw_parts(msg, ret) };
        buf.extend_from_slice(bytes);
        unsafe { nanomsg_sys::nn_freemsg(msg as *mut c_void) };
        Ok(ret)
    }

    /// Receive a message from the socket. Copy the message allocated by nanomsg into the buffer on success.
    /// If the data in the message is not valid UTF-8 then an error is returned and buffer is unchanged.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol};
    /// use std::thread;
    /// use std::io::{Read, Write};
    ///
    /// let mut push_socket = Socket::new(Protocol::Push).unwrap();
    /// let mut push_ep = push_socket.bind("ipc:///tmp/read_to_string_doc.ipc").unwrap();
    ///
    /// let mut pull_socket = Socket::new(Protocol::Pull).unwrap();
    /// let mut pull_ep = pull_socket.connect("ipc:///tmp/read_to_string_doc.ipc").unwrap();
    ///
    /// thread::sleep_ms(50);
    ///
    /// match push_socket.write(b"foobar") {
    ///     Ok(..) => println!("Message sent !"),
    ///     Err(err) => panic!("Failed to write to the socket: {}", err)
    /// }
    ///
    /// let mut msg = String::new();
    /// match pull_socket.read_to_string(&mut msg) {
    ///     Ok(_) => {
    ///         println!("Read {} bytes !", msg.len());
    ///         // here we can process the the message stored in `msg`
    ///     },
    ///     Err(err) => panic!("Problem while reading: {}", err)
    /// };
    /// ```
    ///
    /// # Errors
    ///
    /// - `io::ErrorKind::FileNotFound` : The socket is invalid.
    /// - `io::ErrorKind::MismatchedFileTypeForOperation` : The operation is not supported by this socket type.
    /// - `io::ErrorKind::ResourceUnavailable` : The operation cannot be performed on this socket at the moment because socket is not in the appropriate state. This error may occur with socket types that switch between several states.
    /// - `io::ErrorKind::TimedOut` : Individual socket types may define their own specific timeouts. If such timeout is hit this error will be returned.
    /// - `io::ErrorKind::Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `io::ErrorKind::Other` : The library is terminating, or the message is not a valid UTF-8 string.
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        let mut msg : *mut u8 = ptr::null_mut();
        let ret = unsafe {
            nanomsg_sys::nn_recv(self.socket, mem::transmute(&mut msg), nanomsg_sys::NN_MSG, 0)
        };

        io_error_guard!(ret);

        unsafe {
            let ret = ret as usize;
            let bytes = slice::from_raw_parts(msg, ret);
            match str::from_utf8(bytes) {
                Ok(text) => {
                    buf.push_str(text);
                    nanomsg_sys::nn_freemsg(msg as *mut c_void);
                    Ok(ret)
                },
                Err(_) => {
                    nanomsg_sys::nn_freemsg(msg as *mut c_void);
                    Err(io::Error::new(io::ErrorKind::Other, "UTF8 conversion failed !"))
                },
            }
        }
    }

}

impl io::Write for Socket {
    /// The function will send a message containing the data from the buf parameter to the socket.
    /// Which of the peers the message will be sent to is determined by the particular socket type.
    ///
    ///
    /// # Example:
    ///
    /// ```rust
    /// use nanomsg::{Socket, Protocol};
    /// use std::thread;
    /// use std::io::{Read, Write};
    ///
    /// let mut push_socket = Socket::new(Protocol::Push).unwrap();
    /// let mut push_ep = push_socket.bind("ipc:///tmp/write_doc.ipc").unwrap();
    ///
    /// let mut pull_socket = Socket::new(Protocol::Pull).unwrap();
    /// let mut pull_ep = pull_socket.connect("ipc:///tmp/write_doc.ipc").unwrap();
    /// let mut buffer = [0u8; 1024];
    ///
    /// thread::sleep_ms(50);
    ///
    /// match push_socket.write_all(b"foobar") {
    ///     Ok(..) => println!("Message sent !"),
    ///     Err(err) => panic!("Failed to write to the socket: {}", err)
    /// }
    ///
    /// match pull_socket.read(&mut buffer) {
    ///     Ok(count) => {
    ///         println!("Read {} bytes !", count);
    ///         // here we can process the `count` bytes of the message stored in `buffer`
    ///     },
    ///     Err(err) => panic!("Problem while reading: {}", err)
    /// };
    /// ```
    ///
    /// # Error
    ///
    /// - `io::ErrorKind::FileNotFound` : The socket is invalid.
    /// - `io::ErrorKind::MismatchedFileTypeForOperation` : The operation is not supported by this socket type.
    /// - `io::ErrorKind::ResourceUnavailable` : The operation cannot be performed on this socket at the moment because socket is not in the appropriate state. This error may occur with socket types that switch between several states.
    /// - `io::ErrorKind::Interrupted` : The operation was interrupted by delivery of a signal before the message was received.
    /// - `io::ErrorKind::TimedOut` : Individual socket types may define their own specific timeouts. If such timeout is hit this error will be returned.
    /// - `io::ErrorKind::Other` : The library is terminating.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let buf_len = buf.len() as size_t;
        let buf_ptr = buf.as_ptr() as *const c_void;
        let ret = unsafe { nanomsg_sys::nn_send(self.socket, buf_ptr, buf_len , 0) };

        io_error_guard!(ret);
        Ok(buf_len as usize)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for Socket {
    /// Closes the socket.
    /// Any buffered inbound messages that were not yet received by the application will be discarded.
    /// The library will try to deliver any outstanding outbound messages for the time specified by `set_linger`.
    /// The call will block in the meantime.
    fn drop(&mut self) {
        unsafe { nanomsg_sys::nn_close(self.socket); }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    use {Socket, Protocol, PollRequest, PollFd, Endpoint, PollInOut, Error};
    use libc::c_int;
    use nanomsg_sys;
    use super::Protocol::*;

    use std::io::{Read, Write};

    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn check_allocate() {
        let msg = Socket::allocate_msg(10).unwrap();
        let allocated_len = msg.len();

        Socket::free_msg(msg).unwrap();

        assert_eq!(10, allocated_len)
    }

    #[test]
    fn bool_to_c_int_sanity() {
        assert_eq!(false as c_int, 0);
        assert_eq!(true as c_int, 1);
    }

    #[test]
    fn initialize_socket() {
        let socket = match Socket::new(Pull) {
            Ok(socket) => socket,
            Err(err) => panic!("{}", err)
        };

        assert!(socket.socket >= 0);

        drop(socket)
    }

    #[test]
    fn bind_socket() {
        let mut socket = match Socket::new(Pull) {
            Ok(socket) => socket,
            Err(err) => panic!("{}", err)
        };

        match socket.bind("ipc:///tmp/bind_socket.ipc") {
            Ok(_) => {},
            Err(err) => panic!("{}", err)
        }

        drop(socket);
    }

    #[test]
    fn bind_and_shutdown() {
        let mut socket = match Socket::new(Pull) {
            Ok(socket) => socket,
            Err(err) => panic!("{}", err)
        };

        let mut endpoint = match socket.bind("ipc:///tmp/bind_and_shutdown.ipc") {
            Ok(endpoint) => endpoint,
            Err(err) => panic!("{}", err)
        };

        endpoint.shutdown();

        drop(socket);
    }

    #[test]
    fn connect_and_shutdown() {
        let mut socket = match Socket::new(Push) {
            Ok(socket) => socket,
            Err(err) => panic!("{}", err)
        };

        let mut endpoint = match socket.connect("ipc:///tmp/bind_and_shutdown.ipc") {
            Ok(endpoint) => endpoint,
            Err(err) => panic!("{}", err)
        };

        endpoint.shutdown();

        drop(socket);
    }

    fn test_create_socket(protocol: Protocol) -> Socket {
        match Socket::new(protocol) {
            Ok(socket) => socket,
            Err(err) => panic!("{}", err)
        }
    }

    fn test_bind(socket: &mut Socket, addr: &str) -> Endpoint {
        match socket.bind(addr) {
            Ok(endpoint) => endpoint,
            Err(err) => panic!("{}", err)
        }
    }

    fn test_connect(socket: &mut Socket, addr: &str) -> Endpoint {
        match socket.connect(addr) {
            Ok(endpoint) => endpoint,
            Err(err) => panic!("{}", err)
        }
    }

    fn test_write(socket: &mut Socket, buf: &[u8]) {
        match socket.write_all(buf) {
            Ok(..) => {},
            Err(err) => panic!("Failed to write to the socket: {}", err)
        }
    }

    fn test_zc_write(socket: &mut Socket, buf: &[u8]) {
        let mut msg = Socket::allocate_msg(buf.len()).unwrap();
        for i in 0..buf.len() {
           msg[i] = buf[i];
        }
        match socket.zc_write(msg) {
            Ok(..) => {},
            Err(err) => panic!("Failed to write to the socket: {}", err)
        }
    }

    fn test_read(socket: &mut Socket, expected: &[u8]) {
        let mut buf = [0u8; 6];
        match socket.read(&mut buf) {
            Ok(len) => {
                assert_eq!(len, 6);
                assert_eq!(buf.as_ref(), expected)
            },
            Err(err) => panic!("{}", err)
        }
    }

    fn test_read_to_string(socket: &mut Socket, expected: &str) {
        let mut text = String::new();
        match socket.read_to_string(&mut text) {
            Ok(_) => assert_eq!(text, expected),
            Err(err) => panic!("{}", err)
        }
    }

    fn test_subscribe(socket: &mut Socket, topic: &str) {
        match socket.subscribe(topic) {
            Ok(_) => {},
            Err(err) => panic!("{}", err)
        }
    }

    #[test]
    fn pipeline() {

        let url = "ipc:///tmp/pipeline.ipc";

        let mut push_socket = test_create_socket(Push);
        let mut push_endpoint = test_bind(&mut push_socket, url);

        let mut pull_socket = test_create_socket(Pull);
        test_connect(&mut pull_socket, url);

        thread::sleep(Duration::from_millis(10));

        test_write(&mut push_socket, b"foobar");
        test_read(&mut pull_socket, b"foobar");

        push_endpoint.shutdown();

        drop(pull_socket);
        drop(push_socket);
    }

    #[test]
    fn read_when_buffer_is_smaller_than_msg_return_buffer_size() {

        let url = "ipc:///tmp/pipeline_truncate.ipc";

        let mut push_socket = test_create_socket(Push);
        let mut push_endpoint = test_bind(&mut push_socket, url);

        let mut pull_socket = test_create_socket(Pull);
        test_connect(&mut pull_socket, url);

        thread::sleep(Duration::from_millis(10));

        test_write(&mut push_socket, b"foobarbar");
        test_read(&mut pull_socket, b"foobar");

        push_endpoint.shutdown();

        drop(pull_socket);
        drop(push_socket);
    }

    #[test]
    fn zero_copy_works() {

        let url = "ipc:///tmp/zero_copy_works.ipc";

        let mut push_socket = test_create_socket(Push);
        let mut push_endpoint = test_bind(&mut push_socket, url);

        let mut pull_socket = test_create_socket(Pull);
        test_connect(&mut pull_socket, url);

        thread::sleep(Duration::from_millis(10));

        test_zc_write(&mut push_socket, b"foobar");
        test_read(&mut pull_socket, b"foobar");

        push_endpoint.shutdown();

        drop(pull_socket);
        drop(push_socket);
    }

    fn test_multithread_pipeline(url: &'static str) {

        // this is required to prevent the sender from being dropped too early
        let finish_line = Arc::new(Barrier::new(3));
        let finish_line_pull = finish_line.clone();
        let finish_line_push = finish_line.clone();

        let push_thread = thread::spawn(move || {
            let mut push_socket = test_create_socket(Push);

            test_bind(&mut push_socket, url);
            test_write(&mut push_socket, b"foobar");

            finish_line_push.wait();
        });

        let pull_thread = thread::spawn(move|| {
            let mut pull_socket = test_create_socket(Pull);

            test_connect(&mut pull_socket, url);
            test_read(&mut pull_socket, b"foobar");

            finish_line_pull.wait();
        });

        finish_line.wait();

        push_thread.join();
        pull_thread.join();
    }

    #[test]
    fn pipeline_mt1() {
        test_multithread_pipeline("ipc:///tmp/pipeline_mt1.ipc")
    }

    #[test]
    fn pipeline_mt2() {
        test_multithread_pipeline("ipc:///tmp/pipeline_mt2.ipc")
    }

    #[test]
    fn pair() {

        let url = "ipc:///tmp/pair.ipc";

        let mut left_socket = test_create_socket(Pair);
        test_bind(&mut left_socket, url);

        let mut right_socket = test_create_socket(Pair);
        test_connect(&mut right_socket, url);

        thread::sleep(Duration::from_millis(10));

        test_write(&mut right_socket, b"foobar");
        test_read(&mut left_socket, b"foobar");

        test_write(&mut left_socket, b"foobaz");
        test_read(&mut right_socket, b"foobaz");

        drop(left_socket);
        drop(right_socket);
    }

    #[test]
    fn connect_push_to_multi_ep() {

        let url1 = "ipc:///tmp/connect_push_to_multi_ep_1.ipc";
        let url2 = "ipc:///tmp/connect_push_to_multi_ep_2.ipc";

        let mut server1 = test_create_socket(Pull);
        test_bind(&mut server1, url1);

        let mut server2 = test_create_socket(Pull);
        test_bind(&mut server2, url2);

        let mut client = test_create_socket(Push);
        test_connect(&mut client, url1);
        test_connect(&mut client, url2);

        thread::sleep(Duration::from_millis(10));

        test_write(&mut client, b"foobar");
        thread::sleep(Duration::from_millis(10));

        let mut read_count = 0;
        let mut block_count = 0;
        let mut buf = [0u8; 6];
        match server1.nb_read(&mut buf) {
            Ok(count) => {
                assert_eq!(count, 6);
                read_count = read_count + 1;
            },
            Err(err) => {
                assert_eq!(err, Error::TryAgain);
                block_count = block_count + 1;
            }
        }
        match server2.nb_read(&mut buf) {
            Ok(count) => {
                assert_eq!(count, 6);
                read_count = read_count + 1;
            },
            Err(err) => {
                assert_eq!(err, Error::TryAgain);
                block_count = block_count + 1;
            }
        }
        assert_eq!(read_count, 1);
        assert_eq!(block_count, 1);
    }

    #[test]
    fn bind_pull_to_multi_ep() {

        let url1 = "ipc:///tmp/bind_pull_to_multi_ep_1.ipc";
        let url2 = "ipc:///tmp/bind_pull_to_multi_ep_2.ipc";

        let mut server = test_create_socket(Pull);
        test_bind(&mut server, url1);
        test_bind(&mut server, url2);

        let mut client1 = test_create_socket(Push);
        test_connect(&mut client1, url1);

        let mut client2 = test_create_socket(Push);
        test_connect(&mut client2, url2);

        thread::sleep(Duration::from_millis(10));

        test_write(&mut client1, b"foobar");
        test_write(&mut client2, b"foobaz");
        thread::sleep(Duration::from_millis(10));

        test_read(&mut server, b"foobar");
        test_read(&mut server, b"foobaz");
    }

    #[test]
    fn bus() {

        let url = "ipc:///tmp/bus.ipc";

        let mut sock1 = test_create_socket(Bus);
        test_bind(&mut sock1, url);

        let mut sock2 = test_create_socket(Bus);
        test_connect(&mut sock2, url);

        let mut sock3 = test_create_socket(Bus);
        test_connect(&mut sock3, url);

        thread::sleep(Duration::from_millis(10));

        let msg = b"foobar";
        test_write(&mut sock1, msg);
        test_read(&mut sock2, msg);
        test_read(&mut sock3, msg);

        drop(sock3);
        drop(sock2);
        drop(sock1);
    }

    #[test]
    fn pubsub() {

        let url = "ipc:///tmp/pubsub.ipc";

        let mut sock1 = test_create_socket(Pub);
        test_bind(&mut sock1, url);

        let mut sock2 = test_create_socket(Sub);
        test_subscribe(&mut sock2, "foo");
        test_connect(&mut sock2, url);

        let mut sock3 = test_create_socket(Sub);
        test_subscribe(&mut sock3, "bar");
        test_connect(&mut sock3, url);

        thread::sleep(Duration::from_millis(100));

        let msg1 = b"foobar";
        test_write(&mut sock1, msg1);
        test_read(&mut sock2, msg1);

        let msg2 = b"barfoo";
        test_write(&mut sock1, msg2);
        test_read(&mut sock3, msg2);

        drop(sock3);
        drop(sock2);
        drop(sock1);
    }

    #[test]
    fn survey() {

        let url = "ipc:///tmp/survey.ipc";

        let mut sock1 = test_create_socket(Surveyor);
        test_bind(&mut sock1, url);

        let mut sock2 = test_create_socket(Respondent);
        test_connect(&mut sock2, url);

        let mut sock3 = test_create_socket(Respondent);
        test_connect(&mut sock3, url);

        sock1.set_send_timeout(250);
        sock1.set_receive_timeout(250);
        sock2.set_send_timeout(250);
        sock2.set_receive_timeout(250);
        sock3.set_send_timeout(250);
        sock3.set_receive_timeout(250);

        thread::sleep(Duration::from_millis(50));

        match sock1.set_survey_deadline(500) {
            Ok(socket) => socket,
            Err(err) => panic!("{}", err)
        };

        let question = b"yesno?";
        test_write(&mut sock1, question);
        test_read(&mut sock2, question);
        test_read(&mut sock3, question);

        let vote = b"may be";
        test_write(&mut sock2, vote);
        test_write(&mut sock3, vote);
        test_read(&mut sock1, vote);
        test_read(&mut sock1, vote);

        drop(sock3);
        drop(sock2);
        drop(sock1);
    }

    #[test]
    fn should_change_linger() {

        let mut socket = test_create_socket(Pair);

        match socket.set_linger(1024) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change linger on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_send_buffer_size() {

        let mut socket = test_create_socket(Pair);

        let size = 64 * 1024;
        match socket.set_send_buffer_size(size) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change send buffer size on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_receive_buffer_size() {

        let mut socket = test_create_socket(Pair);

        let size = 64 * 1024;
        match socket.set_receive_buffer_size(size) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change receive buffer size on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_send_timeout() {

        let mut socket = test_create_socket(Pair);

        match socket.set_send_timeout(-2) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change send timeout on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_receive_timeout() {

        let mut socket = test_create_socket(Pair);

        match socket.set_receive_timeout(200) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change receive timeout on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_reconnect_interval() {

        let mut socket = test_create_socket(Pair);

        match socket.set_reconnect_interval(142) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change reconnect interval on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_max_reconnect_interval() {

        let mut socket = test_create_socket(Pair);

        match socket.set_max_reconnect_interval(666) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change reconnect interval on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_send_priority() {

        let mut socket = test_create_socket(Pair);

        match socket.set_send_priority(15u8) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change send priority on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_receive_priority() {

        let mut socket = test_create_socket(Pair);

        match socket.set_receive_priority(2u8) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change receive priority on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_ipv4_only() {

        let mut socket = test_create_socket(Pair);

        match socket.set_ipv4_only(true) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change ipv4 only on the socket: {}", err)
        }

        drop(socket)
    }

    #[cfg(not(windows))]
    #[test]
    fn should_change_socket_name() {

        let mut socket = test_create_socket(Pair);

        match socket.set_socket_name("bob") {
            Ok(..) => {},
            Err(err) => panic!("Failed to change the socket name: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_request_resend_interval() {

        let mut socket = test_create_socket(Req);

        match socket.set_request_resend_interval(60042) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change request resend interval on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_change_tcp_nodelay() {

        let mut socket = test_create_socket(Pair);

        match socket.set_tcp_nodelay(true) {
            Ok(..) => {},
            Err(err) => panic!("Failed to change tcp nodelay only on the socket: {}", err)
        }

        drop(socket)
    }

    #[test]
    fn should_get_receive_fd() {
        let mut socket = test_create_socket(Pair);

        match socket.get_receive_fd() {
            Ok(..) => {},
            Err(err) => panic!("Failed to get receive file descriptor: {}", err)
        }
    }

    #[test]
    fn should_get_send_fd() {
        let mut socket = test_create_socket(Pair);

        match socket.get_send_fd() {
            Ok(..) => {},
            Err(err) => panic!("Failed to get send file descriptor: {}", err)
        }
    }

    #[cfg(not(windows))] // Crashes during appveyor build
    #[test]
    fn should_get_socket_name() {
        let mut socket = test_create_socket(Pair);

        match socket.set_socket_name("bob") {
            Ok(..) => {},
            Err(err) => panic!("Failed to change the socket name: {}", err)
        }

        match socket.get_socket_name(1024) {
            Ok(..) => {},
            Err(err) => panic!("Failed to get socket name: {}", err)
        }
    }

    #[test]
    fn protocol_matches_raw() {
         assert_eq!(nanomsg_sys::NN_REQ, Req.to_raw());
         assert_eq!(nanomsg_sys::NN_REP, Rep.to_raw());
         assert_eq!(nanomsg_sys::NN_PUSH, Push.to_raw());
         assert_eq!(nanomsg_sys::NN_PULL, Pull.to_raw());
         assert_eq!(nanomsg_sys::NN_PAIR, Pair.to_raw());
         assert_eq!(nanomsg_sys::NN_BUS, Bus.to_raw());
         assert_eq!(nanomsg_sys::NN_PUB, Pub.to_raw());
         assert_eq!(nanomsg_sys::NN_SUB, Sub.to_raw());
         assert_eq!(nanomsg_sys::NN_SURVEYOR, Surveyor.to_raw());
         assert_eq!(nanomsg_sys::NN_RESPONDENT, Respondent.to_raw());
    }

    #[test]
    fn test_read_to_end() {

        let url = "ipc:///tmp/read_to_end.ipc";

        let mut left_socket = test_create_socket(Pair);
        test_bind(&mut left_socket, url);

        let mut right_socket = test_create_socket(Pair);
        test_connect(&mut right_socket, url);

        thread::sleep(Duration::from_millis(10));

        test_write(&mut right_socket, b"ok");
        test_read_to_string(&mut left_socket, "ok".as_ref());

        test_write(&mut left_socket, b"not ok");
        test_read_to_string(&mut right_socket, "not ok".as_ref());

        drop(left_socket);
        drop(right_socket);
    }

    #[test]
    fn nb_read_works_in_both_cases() {

        let url = "ipc:///tmp/nb_read_works_in_both_cases.ipc";

        let mut push_socket = test_create_socket(Push);
        test_bind(&mut push_socket, url);

        let mut pull_socket = test_create_socket(Pull);
        test_connect(&mut pull_socket, url);
        thread::sleep(Duration::from_millis(10));

        let mut buf = [0u8; 6];
        match pull_socket.nb_read(&mut buf) {
            Ok(_) => panic!("Nothing should have been received !"),
            Err(err) => assert_eq!(err, Error::TryAgain)
        }

        test_write(&mut push_socket, b"foobar");
        thread::sleep(Duration::from_millis(10));

        let mut buf = [0u8; 6];
        match pull_socket.nb_read(&mut buf) {
            Ok(len) => {
                assert_eq!(len, 6);
                assert_eq!(buf.as_ref(), b"foobar")
            },
            Err(err) => panic!("{}", err)
        }

        drop(pull_socket);
        drop(push_socket);
    }

    #[test]
    fn nb_read_to_end_works_in_both_cases() {

        let url = "ipc:///tmp/nb_read_to_end_works_in_both_cases.ipc";

        let mut push_socket = test_create_socket(Push);
        test_bind(&mut push_socket, url);

        let mut pull_socket = test_create_socket(Pull);
        test_connect(&mut pull_socket, url);
        thread::sleep(Duration::from_millis(10));

        let mut buffer = Vec::new();
        match pull_socket.nb_read_to_end(&mut buffer) {
            Ok(_) => panic!("Nothing should have been received !"),
            Err(err) => assert_eq!(err, Error::TryAgain)
        }

        test_write(&mut push_socket, b"foobar");
        thread::sleep(Duration::from_millis(10));

        let mut buffer = Vec::new();
        match pull_socket.nb_read_to_end(&mut buffer) {
            Ok(_) => {
                assert_eq!(buffer.len(), 6);
                //let buffer = &buffer as &AsRef<[u8]>;
                //assert_eq!(buffer.as_ref(), b"foobar")
                assert_eq!(AsRef::<[u8]>::as_ref(&buffer), b"foobar")
            },
            Err(err) => panic!("{}", err)
        }

        drop(pull_socket);
        drop(push_socket);
    }

    #[test]
    fn nb_write_works_in_both_cases() {

        let url = "ipc:///tmp/nb_write_works_in_both_cases.ipc";

        let mut push_socket = test_create_socket(Push);
        test_bind(&mut push_socket, url);
        thread::sleep(Duration::from_millis(10));

        match push_socket.nb_write(b"barfoo") {
            Ok(_) => panic!("Nothing should have been sent !"),
            Err(err) => assert_eq!(err, Error::TryAgain)
        }

        drop(push_socket);
    }

    #[test]
    fn poll_works() {
        let url = "ipc:///tmp/poll_works_.ipc";

        let mut left_socket = test_create_socket(Pair);
        test_bind(&mut left_socket, url);

        let mut right_socket = test_create_socket(Pair);
        test_connect(&mut right_socket, url);

        thread::sleep(Duration::from_millis(10));

        let pollfd1 = left_socket.new_pollfd(PollInOut::InOut);
        let pollfd2 = right_socket.new_pollfd(PollInOut::InOut);

        // TODO : find some simpler/shorter/better way to intialize a poll request
        let mut pollreq_vector: Vec<PollFd> = vec![pollfd1, pollfd2];
        let mut pollreq_slice = &mut pollreq_vector[..];
        let mut request = PollRequest::new(pollreq_slice);
        let timeout = 10;
        {
            let poll_result = Socket::poll(&mut request, timeout);

            match poll_result {
                Ok(count) => assert_eq!(2, count),
                Err(err) => panic!("Failed to poll: {}", err)
            }

            let fds = request.get_fds();
            assert_eq!(true, fds[0].can_write());
            assert_eq!(false, fds[0].can_read());
            assert_eq!(true, fds[1].can_write());
            assert_eq!(false, fds[1].can_read());
        }

        test_write(&mut right_socket, b"foobar");
        thread::sleep(Duration::from_millis(10));
        {
            let poll_result = Socket::poll(&mut request, timeout);

            match poll_result {
                Ok(count) => assert_eq!(2, count),
                Err(err) => panic!("Failed to poll: {}", err)
            }

            let fds = request.get_fds();
            assert_eq!(true, fds[0].can_write());
            assert_eq!(true, fds[0].can_read()); // and now right socket can read the msg sent by left
            assert_eq!(true, fds[1].can_write());
            assert_eq!(false, fds[1].can_read());
        }
    }
}
