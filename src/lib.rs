// ======================================
// nanomsg.rs : nanomsg bindings for rust
//
// This aims to be a rust version of the
// full public API of nanomsg. But parts
// are probably still missing, since the
// safe API only does nn_send and nn_recv
// currently.
// ======================================

#![crate_type = "lib"]
#![license = "MIT/ASL2"]
#![feature(globs, unsafe_destructor, phase)]
#![allow(dead_code, non_camel_case_types)]

#[phase(plugin, link)] extern crate log;
#[phase(plugin, link)] extern crate shiny;

extern crate libc;
extern crate debug;


use std::ptr;
use std::ptr::RawPtr;
use libc::{c_void,size_t,c_int,c_short,malloc,free};
use std::intrinsics;
use std::mem::transmute;
use std::io;
use std::io::{Reader,Writer};
use std::io::IoResult;
use std::cmp::min;
use std::os::last_os_error;
use std::os::errno;
use std::slice;
use std::num::FromPrimitive;
use std::slice::raw::buf_as_slice;

mod ffi;

pub static AF_SP: c_int = 1;
pub static AF_SP_RAW: c_int = 2;
pub static NN_CHUNKREF_MAX: c_int = 32;
pub static NN_DOMAIN: c_int = 12;
pub static NN_DONTWAIT: c_int = 1;
pub static NN_FSM_ACTION: c_int = -2;
pub static NN_FSM_START: c_int = -2;
pub static NN_FSM_STOP: c_int = -3;
pub static NN_HAUSNUMERO: int = 156384712;
pub static NN_INPROC: c_int = -1;
pub static NN_IPC: c_int = -2;
pub static NN_IPV4ONLY: c_int = 14;
pub static NN_LINGER: c_int = 1;
pub static NN_PIPEBASE_PARSED: c_int = 2;
pub static NN_PIPEBASE_RELEASE: c_int = 1;
pub static NN_PIPE_IN: c_int = 33987;
pub static NN_PIPE_OUT: c_int = 33988;
pub static NN_PIPE_PARSED: c_int = 2;
pub static NN_PIPE_RELEASE: c_int = 1;
pub static NN_PROTO_BUS: c_int = 7;
pub static NN_PROTOCOL: c_int = 13;
pub static NN_PROTO_PAIR: c_int = 1;
pub static NN_PROTO_PIPELINE: c_int = 5;
pub static NN_PROTO_PUBSUB: c_int = 2;
pub static NN_PROTO_REQREP: c_int = 3;
pub static NN_PROTO_SURVEY: c_int = 6;
pub static NN_RCVBUF: c_int = 3;
pub static NN_RCVFD: c_int = 11;
pub static NN_RCVTIMEO: c_int = 5;
pub static NN_RECONNECT_IVL: c_int = 6;
pub static NN_RECONNECT_IVL_MAX: c_int = 7;
pub static NN_REQ_RESEND_IVL: c_int = 1;
pub static NN_SNDBUF: c_int = 2;
pub static NN_SNDFD: c_int = 10;
pub static NN_SNDPRIO: c_int = 8;
pub static NN_SNDTIMEO: c_int = 4;
pub static NN_SOCKADDR_MAX: c_int = 128;
pub static NN_SOCKBASE_EVENT_IN: c_int = 1;
pub static NN_SOCKBASE_EVENT_OUT: c_int = 2;
pub static NN_SOCKTYPE_FLAG_NORECV: c_int = 1;
pub static NN_SOCKTYPE_FLAG_NOSEND: c_int = 2;
pub static NN_SOL_SOCKET: c_int = 0;
pub static NN_SUB_SUBSCRIBE: c_int = 1;
pub static NN_SUB_UNSUBSCRIBE: c_int = 2;
pub static NN_SURVEYOR_DEADLINE: c_int = 1;
pub static NN_TCP: c_int = -3;
pub static NN_TCP_NODELAY: c_int = 1;
pub static NN_VERSION_AGE: c_int = 0;
pub static NN_VERSION_CURRENT: c_int = 0;
pub static NN_VERSION_REVISION: c_int = 0;
pub static NN_POLLIN: c_short = 1;
pub static NN_POLLOUT: c_short = 2;

pub static NN_BUS: c_int = (NN_PROTO_BUS * 16 + 0);
pub static NN_MSG: u64 = -1;
pub static NN_PAIR: c_int = (NN_PROTO_PAIR * 16 + 0);
pub static NN_PUSH: c_int = (NN_PROTO_PIPELINE * 16 + 0);
pub static NN_PULL: c_int = (NN_PROTO_PIPELINE * 16 + 1);
pub static NN_PUB: c_int = (NN_PROTO_PUBSUB * 16 + 0);
pub static NN_SUB: c_int = (NN_PROTO_PUBSUB * 16 + 1);
pub static NN_REQ: c_int = (NN_PROTO_REQREP * 16 + 0);
pub static NN_REP: c_int = (NN_PROTO_REQREP * 16 + 1);
pub static NN_SURVEYOR: c_int = (NN_PROTO_SURVEY * 16 + 0);
pub static NN_RESPONDENT: c_int = (NN_PROTO_SURVEY * 16 + 1);

pub static NN_QUEUE_NOTINQUEUE: c_int = -1;
pub static NN_LIST_NOTINLIST: c_int = -1;

pub type c_schar = i8;

pub struct IoVec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

//MsgHdr
pub struct MsgHdr {
    msg_iov: *mut IoVec,
    msg_iovlen: c_int,
    msg_control: *mut c_void,
    msg_controllen: size_t,
}

#[deriving(Show)]
pub struct PollFd {
    fd: c_int,
    events: c_short,
    revents: c_short
}

#[link(name = "nanomsg")]
extern "C" {
    pub static mut program_invocation_name: *mut c_schar;

    pub static mut program_invocation_short_name: *mut c_schar;

    pub fn __errno_location() -> *mut c_int;

    pub fn nn_errno() -> c_int;

    pub fn nn_strerror(errnum: c_int) -> *const c_schar;

    pub fn nn_symbol(i: c_int,
    value: *mut c_int) -> *const c_schar;

    pub fn nn_term();

    pub fn nn_allocmsg(size: size_t,
    _type: c_int) -> *mut c_void;

    pub fn nn_freemsg(msg: *mut c_void) -> c_int;

    pub fn nn_socket(domain: c_int, protocol: c_int) -> c_int;

    pub fn nn_close(s: c_int) -> c_int;

    pub fn nn_setsockopt(s: c_int,
    level: c_int,
    option: c_int,
    optval: *const c_void,
    optvallen: size_t) -> c_int;

    pub fn nn_getsockopt(s: c_int, level: c_int,
    option: c_int,
    optval: *mut c_void,
    optvallen: *mut size_t) -> c_int;

    pub fn nn_bind(s: c_int, addr: *const c_schar) -> c_int;

    pub fn nn_connect(s: c_int, addr: *const c_schar) -> c_int;

    pub fn nn_shutdown(s: c_int, how: c_int) -> c_int;

    pub fn nn_send(s: c_int,
    buf: *const c_void,
    len: size_t,
    flags: c_int) -> c_int;

    pub fn nn_recv(s: c_int,
    buf: *mut c_void,
    len: size_t,
    flags: c_int) -> c_int;

    pub fn nn_sendmsg(s: c_int,
    msghdr: *const MsgHdr,
    flags: c_int) -> c_int;

    pub fn nn_recvmsg(s: c_int,
    msghdr: *mut MsgHdr,
    flags: c_int) -> c_int;

    pub fn nn_device(s1: c_int,
    s2: c_int) -> c_int;

    pub fn nn_poll(fds: *mut PollFd, nfds: c_int, timeout: c_int) -> c_int;
}

#[deriving(Show)]
pub struct NanoErr {
    pub rc: c_int,
    pub errstr: String,
}

pub struct NanoSocket {
    sock: c_int,
}

#[deriving(PartialEq, FromPrimitive, Show)]
pub enum NanoError {
    ENOTSUP = NN_HAUSNUMERO + 1,
    EPROTONOSUPPORT = NN_HAUSNUMERO + 2,
    ENOBUFS = NN_HAUSNUMERO + 3,
    ENETDOWN = NN_HAUSNUMERO + 4,
    EADDRINUSE = NN_HAUSNUMERO + 5,
    EADDRNOTAVAIL = NN_HAUSNUMERO + 6,
    ECONNREFUSED = NN_HAUSNUMERO + 7,
    EINPROGRESS = NN_HAUSNUMERO + 8,
    ENOTSOCK = NN_HAUSNUMERO + 9,
    EAFNOSUPPORT = NN_HAUSNUMERO + 10,
    EPROTO = NN_HAUSNUMERO + 11,
    EAGAIN = NN_HAUSNUMERO + 12,
    EBADF = NN_HAUSNUMERO + 13,
    EINVAL = NN_HAUSNUMERO + 14,
    EMFILE = NN_HAUSNUMERO + 15,
    EFAULT = NN_HAUSNUMERO + 16,
    EACCESS = NN_HAUSNUMERO + 17,
    ENETRESET = NN_HAUSNUMERO + 18,
    ENETUNREACH = NN_HAUSNUMERO + 19,
    EHOSTUNREACH = NN_HAUSNUMERO + 20,
    ENOTCONN = NN_HAUSNUMERO + 21,
    EMSGSIZE = NN_HAUSNUMERO + 22,
    ETIMEDOUT = NN_HAUSNUMERO + 23,
    ECONNABORTED = NN_HAUSNUMERO + 24,
    ECONNRESET = NN_HAUSNUMERO + 25,
    ENOPROTOOPT = NN_HAUSNUMERO + 26,
    EISCONN = NN_HAUSNUMERO + 27,
    ETIMEOUT = NN_HAUSNUMERO + 100, // Added by library for timeouts
    EUNKNOWN = NN_HAUSNUMERO + 101  // Added by library for unknown problems
}

impl NanoSocket {

    // example: let sock = NanoSocket::new(AF_SP, NN_PAIR);
    #[inline(never)]
    pub fn new(domain: c_int, protocol: c_int) -> Result<NanoSocket, NanoErr> {

        let rc: c_int = unsafe { nn_socket(domain, protocol) };
        if rc < 0 {
            Err(NanoErr{ rc: rc, errstr: last_os_error() })
        } else {
            Ok(NanoSocket{ sock: rc })
        }
    }

    // connect
    #[inline(never)]
    pub fn connect(&self, addr: &str) -> Result<(), NanoErr> {

        let rc = unsafe { nn_connect(self.sock, addr.to_c_str().as_ptr()) };
        if rc < 0 {
            Err(NanoErr{ rc: rc, errstr: last_os_error() })
        } else {
            Ok(())
        }
    }

    // bind(listen)
    #[inline(never)]
    pub fn bind(&self, addr: &str) -> Result<(), NanoErr>{

        // bind
        let rc = unsafe { nn_bind(self.sock, addr.to_c_str().as_ptr()) };
        if rc < 0 {
            Err(NanoErr{rc: rc, errstr: last_os_error() })
        } else {
            Ok(())
        }
    }

    // subscribe, with prefix-filter
    #[inline(never)]
    pub fn subscribe(&self, prefix: &[u8]) -> Result<(), NanoErr>{

        let rc = unsafe {
            nn_setsockopt(self.sock,
            NN_SUB,
            NN_SUB_SUBSCRIBE,
            prefix.as_ptr() as *const c_void,
            prefix.len() as u64)
        };

        if rc < 0 {
            Err( NanoErr{ rc: rc, errstr: last_os_error() })
        } else {
            Ok(())
        }
    }
    /*
       pub fn getFd(&self, FdType) -> Result<fd_t, NanoErr>{

       }
       */
    // send
    #[inline(never)]
    pub fn send(&self, buf: &[u8]) -> Result<(), NanoErr> {

        let len : i64 = buf.len() as i64;
        if 0 == len { return Ok(()); }

        let rc : i64 = unsafe { nn_send(self.sock, buf.as_ptr() as *const c_void, len as u64, 0) } as i64;

        if rc < 0 {
            Err(NanoErr{rc: rc as i32, errstr: last_os_error() })
        } else {
            Ok(())
        }
    }

    // send a string
    #[inline(never)]
    pub fn sendstr(&self, b: &str) -> Result<(), NanoErr> {

        let len = b.len();
        if 0 == len { return Ok(()); }

        let rc = unsafe { nn_send(self.sock, b.to_c_str().as_ptr() as *const libc::c_void, len as u64, 0) };

        if rc < 0 {
            Err(NanoErr{rc: rc as i32, errstr: last_os_error() })
        } else {
            Ok(())
        }
    }

    // buffer receive
    #[inline(never)]
    pub fn recv(&self) -> Result<Vec<u8>, NanoErr> {

        let mut mem : *mut u8 = ptr::mut_null();
        let recvd = unsafe { nn_recv(self.sock, transmute(&mut mem), NN_MSG, 0) };

        if recvd < 0 {
            Err(NanoErr{rc: recvd as i32, errstr: last_os_error() })
        } else {
            unsafe {
                let buf = std::slice::raw::buf_as_slice(mem as *const u8, recvd as uint, |buf| {
                    buf.to_vec()
                });
                nn_freemsg(mem as *mut c_void);
                Ok(buf)
            }
        }
    }

    #[inline(never)]
    pub fn getsockopt(&self, level: i32, option: i32) -> Result<u32, NanoErr> {

        let mut optval: u32 = 0;
        let optval_ptr: *mut u32 = &mut optval;

        let mut optvallen: u64 = 4;
        let optvallen_ptr: *mut u64 = &mut optvallen;

        let recvd = unsafe {
            nn_getsockopt(self.sock,
            level,
            option,
            optval_ptr as *mut c_void,
            optvallen_ptr) as i64
        };

        match recvd {
            0 => Ok(optval),
            _ => Err(NanoErr{ rc: recvd as i32, errstr: last_os_error() })
        }
    }

    #[inline(never)]
    pub fn can_send(&self, timeout: int) -> bool {
        match self.poll(true, false, timeout) {
            Ok((s, _)) => s,
            Err(_) => false
        }
    }

    #[inline(never)]
    pub fn can_receive(&self, timeout: int) -> bool {
        match self.poll(false, true, timeout) {
            Ok((_, r)) => r,
            Err(_) => false
        }
    }

    #[inline(never)]
    pub fn poll(&self, send: bool, receive: bool, timeout: int) -> Result<(bool, bool), NanoError> {
        let events: i16 = match (send, receive) {
            (false, false) => return Ok((false,false)),  // If you don't want to poll either, exit early
                (true, false)  => NN_POLLOUT,
                (false, true)  => NN_POLLIN,
                (true, true)   => NN_POLLIN | NN_POLLOUT
        };

        let mut pollfd = PollFd { fd: self.sock, events: events, revents: 0 };
        let pollfds_ptr: *mut PollFd = &mut pollfd;

        let ret = unsafe { nn_poll(pollfds_ptr, 1 as c_int, timeout as i32) };

        drop(pollfds_ptr);

        match ret {
            0 => Err(ETIMEOUT),
            -1 => {
                match self.errno() {
                    Some(s) => Err(s),
                    None => Err(EUNKNOWN)
                }

            },
            _ => {
                let can_send = pollfd.revents & NN_POLLOUT > 0;
                let can_recv = pollfd.revents & NN_POLLIN > 0;
                Ok((can_send, can_recv))
            }
        }
    }

    #[inline(never)]
    pub fn errno(&self) -> Option<NanoError> {
        let error: Option<NanoError> = FromPrimitive::from_i32( unsafe { nn_errno() });
        error
    }
}

impl std::io::Reader for NanoSocket {
    #[inline(never)]
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {

        match self.recv() {
            Err(e) => {
                warn!("recv failed: {:?} {:?}",e.rc, e.errstr);
                // [TODO]: Return specific error based on the failure.
                Err(io::standard_error(io::OtherIoError))
            },
            Ok(b) => {
                let copylen = min(b.len(), buf.len());
                slice::bytes::copy_memory(buf, b.slice(0, copylen));
                Ok(copylen)
            }
        }
    }
}

impl io::Seek for NanoSocket {
    fn seek(&mut self, _offset: i64, _whence: io::SeekStyle) -> IoResult<()> {
        Err(io::standard_error(io::OtherIoError))
    }
    fn tell(&self) -> IoResult<u64> {fail!();}
}

impl std::io::Writer for NanoSocket {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        // [TODO]: Return specific error based on the failure.
        self.send(buf).map_err(|_| io::standard_error(io::OtherIoError))
    }
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

#[unsafe_destructor]
impl Drop for NanoSocket {
    #[inline(never)]
    fn drop(&mut self) {

        // close
        let rc = unsafe { nn_close(self.sock) };
        if rc != 0 {
            let msg = format!("nn_close({:?}) failed with errno: {:?} '{:?}'", self.sock, std::os::errno(), std::os::last_os_error());
            error!("{:s}", msg);
            fail!("{:s}", msg);
        }
    }
}

// ======================================================
// Nanomsg
//
//  It is not necessary to restrict the lifetime of the
//  Nanomsg to be a subset of the lifetime of the
//  NanoSocket. But if you wanted to do that, you would
//  write:
//     struct Msg<'self> { socket: &'self Socket, ... }
//  which would put a borrowed pointer to the socket in
//  the msg and restrict the messages' lifetime.
// ======================================================

enum HowToCleanup {
    ///  depending on whether recv_any_size() or recv_no_more_than_maxlen()
    ///  was used to get the message, the cleanup code is different.
    ///  If recv_any_size (zero-copy enabled), then we call nn_freemsg().
    ///  if recv_no_more_than_maxlen(), the we call ::free() to release the malloc.
    ///  Lastly if we have no message to cleanup, then DoNothing.
    Free,
    FreeMsg,
    DoNothing
}

/// a wrapper around the message returned by nn_recv
pub struct Nanomsg {
    buf: *mut u8,
    bytes_stored_in_buf: u64,
    bytes_available: u64,
    cleanup: HowToCleanup,
}

impl Nanomsg {

    pub fn new() -> Nanomsg {
        let buf : *mut u8 = ptr::mut_null();
        Nanomsg{buf: buf, bytes_stored_in_buf: 0, bytes_available: 0, cleanup: DoNothing }
    }

    pub fn len(&self) -> u64 {
        self.bytes_stored_in_buf
    }

    pub fn actual_msg_bytes_avail(&self) -> u64 {
        self.bytes_available
    }

    pub fn printbuf(&self) {
        println!("Nanomsg contains message of length {:?}: '{:s}'", self.bytes_stored_in_buf, self.copy_to_string());
    }

    /// Unwraps the Nanomsg.
    /// Any ownership of the message pointed to by buf is forgotten.
    /// Since we take self by value, no further access is possible.
    pub unsafe fn unwrap(self) -> *mut u8 {
        let mut msg = self;
        msg.cleanup = DoNothing;
        msg.buf
    }

    /// recv_any_size allows nanomsg to do zero-copy optimizations
    #[inline(never)]
    pub fn recv_any_size(&mut self, sock: c_int, flags: c_int) -> Result<u64, NanoErr>{

        match self.cleanup {
            DoNothing => (),
            Free => self.cleanup(),
            FreeMsg => self.cleanup()
        }

        let len = unsafe { nn_recv(sock,  transmute(&mut self.buf), NN_MSG, flags) };

        self.bytes_stored_in_buf = len as u64;
        self.bytes_available = self.bytes_stored_in_buf;

        if len < 0 {
            debug!("nn_recv failed with errno: {:?} '{:?}'", std::os::errno(), std::os::last_os_error());
            Err(NanoErr{ rc: std::os::errno() as i32, errstr: last_os_error() })
        } else {
            self.cleanup = FreeMsg;
            Ok(self.bytes_stored_in_buf)
        }
    }

    /// Use recv_no_more_than_maxlen() if we need our own copy anyway, but don't want to overflow our
    /// heap. The function will truncate any part of the message over maxlen. In general, prefer recv_any_size() above.
    #[inline(never)]
    pub fn recv_no_more_than_maxlen(&mut self, sock: c_int, maxlen: u64, flags: c_int) -> Result<u64, NanoErr> {

        match self.cleanup {
            DoNothing => (),
            Free => self.cleanup(),
            FreeMsg => self.cleanup()
        }

        let ptr = unsafe { malloc(maxlen as size_t) as *mut u8 };

        assert!(!ptr.is_null());

        self.cleanup = Free;
        self.buf = ptr;

        let len = unsafe { nn_recv(sock, transmute(self.buf), maxlen, flags) };

        self.bytes_available = len as u64;

        if len < 0 {
            let errmsg = format!("recv_no_more_than_maxlen: nn_recv failed with errno: {:?} '{:?}'", std::os::errno(), std::os::last_os_error());
            warn!("{:s}", errmsg);
            return Err(NanoErr{rc: std::os::errno() as i32, errstr: last_os_error() });
        }

        if self.bytes_available > maxlen {
            let errmsg = format!("recv_no_more_than_maxlen: message was longer ({:?} bytes) than we allocated space for ({:?} bytes)", self.bytes_available, maxlen);
            warn!("{:s}", errmsg);
        }

        self.bytes_stored_in_buf = min(maxlen, self.bytes_available);
        Ok(self.bytes_stored_in_buf)
    }

    pub fn copy_to_string(&self) -> String {
        unsafe { std::string::raw::from_buf_len(self.buf as *const u8, self.bytes_stored_in_buf as uint) }
    }

    #[inline(never)]
    pub fn cleanup(&self) {

        if self.buf.is_null() { return; }

        match self.cleanup {
            DoNothing => (),
            Free => {
                unsafe {
                    // see example code: http://static.rust-lang.org/doc/tutorial-ffi.html

                    let x = intrinsics::init(); // dummy value to swap in
                    // moving the object out is needed to call the destructor
                    ptr::replace(self.buf, x);
                    free(self.buf as *mut c_void)
                }
            },

            FreeMsg => {
                unsafe {
                    let x = intrinsics::init(); // dummy value to swap in
                    // moving the object out is needed to call the destructor
                    ptr::replace(self.buf, x);

                    let rc = nn_freemsg(self.buf as *mut c_void);
                    assert_eq!(rc, 0);
                }
            }
        }
    }
}

#[unsafe_destructor]
impl Drop for Nanomsg {
    #[inline(never)]
    fn drop(&mut self) {
        self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use)]
    extern crate debug;

    use super::*;
    use std::io::timer::sleep;
    use std::comm;


    #[test]
    fn should_poll() {
        let addr="tcp://127.0.0.1:8899";

        let mut sock = NanoSocket::new(AF_SP, NN_PAIR).unwrap();

        sock.bind(addr);

        let (parent_send, child_recv) = comm::channel::<int>();
        let (child_send, parent_recv) = comm::channel::<int>();

        spawn(proc() {
            let addr="tcp://127.0.0.1:8899";

            let sock = NanoSocket::new(AF_SP, NN_PAIR).unwrap();

            sock.connect(addr);

            parent_send.send(0);
            parent_recv.recv();

            let (can_send, can_recv) = match sock.poll(true, true, 1000) {
                Ok((s, r)) => (s,r),
                Err(e) => fail!(format!("Failed: {}", e))
            };

            // Can send since we are connected
            assert!(can_send);

            // Cannot read, since no messages are pending
            assert_eq!(can_recv, false);

            // Send two batches of messages
            sock.send([0,0,0,0,0]);
            sock.send([0,0,0,0,0]);

            parent_send.send(0);

            // Signal to shutdown
            parent_recv.recv();
        });

        // Binding has completed
        child_recv.recv();

        let (can_send, can_recv) = match sock.poll(true, true, 1000) {
            Ok((s, r)) => (s,r),
            Err(e) => fail!("Failed: {}", e)
        };

        // Can send since we are connected.
        assert!(can_send);

        // Cannot read, since no messages are pending.
        assert!(!can_recv);

        child_send.send(0);

        // Two messages pending
        child_recv.recv();

        // hacky, but sometimes the socket send doesnt finish before the chan send.
        sleep(100);

        let (can_send, can_recv) = match sock.poll(true, true, 1000) {
            Ok((s, r)) => (s,r),
            Err(e) => fail!("Failed: {}", e)
        };

        assert!(can_send);

        // Can now read since two messages are pending
        assert!(can_recv);

        // Read the first batch
        let mut buf = [0,0,0,0,0];

        sock.read(buf);

        let (can_send, can_recv) = match sock.poll(true, true, 1000) {
            Ok((s, r)) => (s,r),
            Err(e) => fail!("Failed: {}", e)
        };

        assert!(can_send);

        // Should still be true, one message pending
        assert!(can_recv);

        // One message pending
        // Read second message
        sock.read(buf);

        let (can_send, can_recv) = match sock.poll(true, true, 1000) {
            Ok((s, r)) => (s,r),
            Err(e) => fail!("Failed: {}", e)
        };

        assert!(can_send);

        // Can no longer read since all messages have been received
        assert_eq!(can_recv, false);

        child_send.send(0);
    }

    #[test]
    fn should_send_a_buffer_and_receive_it_correctly() {
        let addr = "tcp://127.0.0.1:1234";

        spawn(proc() {
            let mut sock = NanoSocket::new(AF_SP, NN_PAIR).unwrap();

            sock.bind(addr);

            let mut buf = [0,0,0,0,0];

            sock.read(buf);

            assert_eq!(buf[0], 1);
            assert_eq!(buf[1], 2);
            assert_eq!(buf[2], 3);
            assert_eq!(buf[3], 4);
        });

        let mut sock = NanoSocket::new(AF_SP, NN_PAIR).unwrap();

        sock.connect(addr);
        sock.write([1,2,3,4]);
    }

    #[test]
    fn should_test_sockopts() {
        let addr="tcp://127.0.0.1:8898";
        let sock = NanoSocket::new(AF_SP, NN_PAIR).unwrap();

        sock.bind(addr);

        // Linger default is 1000ms
        let ret = match sock.getsockopt(NN_SOL_SOCKET, NN_LINGER) {
            Ok(s) => s,
            Err(e) => fail!("failed getsockopt(NN_SOL_SOCKET, NN_LINGER) with err: {}", e)
        };

        assert_eq!(ret, 1000);

        // SendBuffer default is 128kb (131072 bytes)
        let ret = match sock.getsockopt(NN_SOL_SOCKET, NN_SNDBUF) {
            Ok(s) => s,
            Err(e) => fail!("failed getsockopt(NN_SOL_SOCKET, NN_SNDBUF) with err: {}", e)
        };

        assert_eq!(ret, 131072);

        // ReceiveBuffer default is 128kb (131072 bytes)
        let ret = match sock.getsockopt(NN_SOL_SOCKET, NN_RCVBUF) {
            Ok(s) => s,
            Err(e) => fail!("failed getsockopt(NN_SOL_SOCKET, NN_RCVBUF) with err: {}", e)
        };

        assert_eq!(ret, 131072);

        // Send timeout default is -1 (unlimited)
        let ret = match sock.getsockopt(NN_SOL_SOCKET, NN_SNDTIMEO) {
            Ok(s) => s,
            Err(e) => fail!("failed getsockopt(NN_SOL_SOCKET, NN_SNDTIMEO) with err: {}", e)
        };

        assert_eq!(ret, -1);
    }

    #[test]
    fn smoke_test_msg_client_msg_server() {

        spawn(proc() {
            msgserver_test();
        });

        msgclient_test();
    }

    fn msgclient_test() {
        let mut msg = Nanomsg::new();

        let address = "tcp://127.0.0.1:5439";

        let sock = match NanoSocket::new(AF_SP, NN_PAIR) {
            Ok(s) => s,
            Err(e) => fail!("Failed with err: {}", e)
        };

        sock.connect(address);

        let b = "WHY";
        sock.sendstr(b);

        match msg.recv_any_size(sock.sock, 0) {
            Ok(_) => {
                let m = msg.copy_to_string();
                assert_eq!(m.as_slice(), "LUV");
            },
            Err(e) => fail!("recv_any_size -> nn_recv failed with errno: {}", e)
        }

        // it is okay to reuse msg (e.g. as below, or in a loop). Nanomsg will free any previous message before
        // receiving a new one. Demonstrate Nanomsg::recv_no_more_than_maxlen()
        match msg.recv_no_more_than_maxlen(sock.sock, 2, 0) {
            Ok(_) => {
                let m = msg.copy_to_string();
                assert_eq!(m.as_slice(), "CA");
            },
            Err(e) => fail!("recv_no_more_than_maxlen -> nn_recv failed with errno: {}", e)
        }
    }

    fn msgserver_test() {
        let mut msg = Nanomsg::new();
        let address = "tcp://127.0.0.1:5439";

        let sock = match NanoSocket::new(AF_SP, NN_PAIR) {
            Ok(s) => s,
            Err(e) => fail!("Failed with err: {}", e)
        };

        match sock.bind(address) {
            Ok(_) => {},
            Err(e) => fail!("Bind failed with err: {}", e)
        }

        match msg.recv_any_size(sock.sock, 0) {
            Ok(_) => {
                let m = msg.copy_to_string();
                assert_eq!(m.as_slice(), "WHY");
            },
            Err(e) => fail!("recv_any_size -> nn_recv failed with errno: {}", e)
        }

        let b = "LUV";
        match sock.sendstr(b) {
            Ok(_) => {},
            Err(e) => fail!("send failed with err: {}", e)
        }

        let b = "CAT";
        match sock.sendstr(b) {
            Ok(_) => {},
            Err(e) =>{
                fail!("send failed with err: {}", e);
            }
        }
    }
}
