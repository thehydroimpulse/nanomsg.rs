// ======================================
// nanomsg.rs : nanomsg bindings for rust
//
// This aims to be a rust version of the
// full public API of nanomsg. But parts
// are probably still missing, since the
// safe API only does nn_send and nn_recv
// currently.
// ======================================

#![crate_id = "nanomsg#0.02"]
#![crate_type = "lib"]
#![license = "MIT/ASL2"]
#![feature(globs)]
#![allow(unused_must_use,dead_code,non_camel_case_types)]

#![feature(phase)]
#[phase(syntax, link)] extern crate log;

extern crate libc;

use std::ptr;
use std::ptr::RawPtr;
use libc::{c_void,size_t,c_int,malloc,free};
use std::intrinsics;
use std::cast::transmute;
use std::io;
use std::io::{Reader,Writer};
use std::io::IoResult;
use std::cmp::min;
use std::os::last_os_error;
use std::os::errno;
use std::slice;

pub static AF_SP: c_int = 1;
pub static AF_SP_RAW: c_int = 2;
pub static NN_CHUNKREF_MAX: c_int = 32;
pub static NN_DOMAIN: c_int = 12;
pub static NN_DONTWAIT: c_int = 1;
pub static NN_FSM_ACTION: c_int = -2;
pub static NN_FSM_START: c_int = -2;
pub static NN_FSM_STOP: c_int = -3;
pub static NN_HAUSNUMERO: c_int = 156384712;
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
pub static EACCESS: c_int = (NN_HAUSNUMERO + 17);
pub static ETERM: c_int = (NN_HAUSNUMERO + 53);
pub static EFSM: c_int = (NN_HAUSNUMERO + 54);
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

#[link(name = "nanomsg")]
extern "C" {
    pub static mut program_invocation_name: *mut c_schar;

    pub static mut program_invocation_short_name: *mut c_schar;

    pub fn __errno_location() -> *mut c_int;

    pub fn nn_errno() -> c_int;

    pub fn nn_strerror(errnum: c_int) -> *c_schar;

    pub fn nn_symbol(i: c_int,
                     value: *mut c_int) -> *c_schar;

    pub fn nn_term();

    pub fn nn_allocmsg(size: size_t,
                       _type: c_int) -> *mut c_void;

    pub fn nn_freemsg(msg: *mut c_void) -> c_int;

    pub fn nn_socket(domain: c_int, protocol: c_int) -> c_int;

    pub fn nn_close(s: c_int) -> c_int;

    pub fn nn_setsockopt(s: c_int,
                         level: c_int,
                         option: c_int,
                         optval: *c_void,
                         optvallen: size_t) -> c_int;

    pub fn nn_getsockopt(s: c_int, level: c_int,
                         option: c_int,
                         optval: *mut c_void,
                         optvallen: *mut size_t) -> c_int;

    pub fn nn_bind(s: c_int, addr: *c_schar) -> c_int;

    pub fn nn_connect(s: c_int, addr: *c_schar) -> c_int;

    pub fn nn_shutdown(s: c_int, how: c_int) -> c_int;

    pub fn nn_send(s: c_int,
                   buf: *c_void,
                   len: size_t,
                   flags: c_int) -> c_int;

    pub fn nn_recv(s: c_int,
                   buf: *mut c_void,
                   len: size_t,
                   flags: c_int) -> c_int;

    pub fn nn_sendmsg(s: c_int,
                      msghdr: *MsgHdr,
                      flags: c_int) -> c_int;

    pub fn nn_recvmsg(s: c_int,
                      msghdr: *mut MsgHdr,
                      flags: c_int) -> c_int;

    pub fn nn_device(s1: c_int,
                     s2: c_int) -> c_int;
}


// ======================================================
// NanoErr
// ======================================================
pub struct NanoErr {
    pub rc: c_int,
    pub errstr: ~str,
}

// Rust-idiomatic memory safe wrappers for nanomsg objects:

// ======================================================
// NanoSocket
// ======================================================

pub struct NanoSocket {
    sock: c_int,
}

impl NanoSocket {

    // example: let sock = NanoSocket::new(AF_SP, NN_PAIR);
    pub fn new(domain: c_int, protocol: c_int) -> Result<NanoSocket, NanoErr> {
        #![inline(never)]

        let rc: c_int = unsafe { nn_socket(domain, protocol) };
        if rc < 0 {
            return Err(NanoErr{
                rc: rc,
                errstr: last_os_error()
            });
        }

        Ok(NanoSocket{
            sock: rc
        })
    }

    // connect
    pub fn connect(&self, addr: &str) -> Result<(), NanoErr> {
        #![inline(never)]

        let addr_c = addr.to_c_str();
        let rc: c_int = addr_c.with_ref(|a| unsafe { nn_connect(self.sock, a) });
        if rc < 0 {
            return Err(NanoErr{
                rc: rc,
                errstr: last_os_error()
            });
        }

        Ok(())
    }

    // bind (listen)
    pub fn bind(&self, addr: &str) -> Result<(), NanoErr>{
        #![inline(never)]

         // bind
        let addr_c = addr.to_c_str();
        let rc : c_int = addr_c.with_ref(|a| unsafe { nn_bind (self.sock, a) });
        if rc < 0 {
            return Err( NanoErr{rc: rc, errstr: last_os_error() } );
        }
        Ok(())
    }

    // subscribe, with prefix-filter
    pub fn subscribe(&self, prefix: &[u8]) -> Result<(), NanoErr>{
        #![inline(never)]

        unsafe {
            let rc : c_int = nn_setsockopt(self.sock,
                NN_SUB,
                NN_SUB_SUBSCRIBE,
                prefix.as_ptr() as *c_void,
                prefix.len() as u64);
            if rc < 0 {
                return Err( NanoErr{ rc: rc, errstr: last_os_error() });
            }
        }
        Ok(())
    }
/*
    pub fn getFd(&self, FdType) -> Result<fd_t, NanoErr>{

    }
*/
    // send
    pub fn send(&self, buf: &[u8]) -> Result<(), NanoErr> {
        #![inline(never)]

        let len : i64 = buf.len() as i64;
        if 0 == len { return Ok(()); }

        let rc : i64 = unsafe { nn_send (self.sock, buf.as_ptr() as *c_void, len as u64, 0) } as i64;

        if rc < 0 {
            return Err( NanoErr{rc: rc as i32, errstr: last_os_error() } );
        }
        Ok(())
    }


    // send a string
    pub fn sendstr(&self, b: &str) -> Result<(), NanoErr> {
        #![inline(never)]

        let len : i64 = b.len() as i64;
        if 0 == len { return Ok(()); }

        let buf = b.to_c_str();
        let rc : i64 = buf.with_ref(|b| unsafe { nn_send (self.sock, b as *libc::c_void, len as u64, 0) }) as i64;

        if rc < 0 {
            return Err( NanoErr{rc: rc as i32, errstr: last_os_error() } );
        }
        Ok(())
    }


    // buffer receive
    pub fn recv(&self) -> Result<~[u8], NanoErr> {
        #![inline(never)]

        unsafe {
            let mut mem : *mut u8 = ptr::mut_null();
            let recvd = nn_recv (self.sock, transmute(&mut mem), NN_MSG, 0) as i64;

            if recvd < 0 {
                return Err( NanoErr{rc: recvd as i32, errstr: last_os_error() } );
            }

            let buf = slice::raw::from_buf_raw(mem as *u8, recvd as uint);
            nn_freemsg(mem as *mut c_void);
            Ok(buf)
        }
    }


    pub fn getsockopt(&self, level: i32, option: i32) -> Result<u32, NanoErr> {
        #![inline(never)]

        unsafe {
            let mut optval: u32 = 0;
            let mut optval_ptr: *mut u32 = &mut optval;

            let mut optvallen: u64 = 4;
            let mut optvallen_ptr: *mut u64 = &mut optvallen;
            let recvd = nn_getsockopt (self.sock, level, option, optval_ptr as *mut c_void, optvallen_ptr) as i64;

            match recvd {
              0 => Ok(optval),
              _ => return Err( NanoErr{rc: recvd as i32, errstr: last_os_error() } )
            }

        }
    }
}


struct NanoMsgReader {
    sock : NanoSocket,
    msg_off : uint,
    flags : uint
}

impl std::io::Reader for NanoSocket {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        #![inline(never)]

        match self.recv() {
            Err(e) => {
                warn!("recv failed: {:?} {:?}",e.rc, e.errstr)
                // [TODO]: Return specific error based on the failure.
                return Err(io::standard_error(io::OtherIoError));
            },
            Ok(b) => {
                let copylen = min(b.len(), buf.len());
                slice::bytes::copy_memory(buf, b.slice(0, copylen));
                return Ok(copylen);
            }
        }
    }
}


impl io::Seek for NanoSocket {
    fn seek(&mut self, _offset: i64, _whence: io::SeekStyle) -> IoResult<()> {
      return Err(io::standard_error(io::OtherIoError));
    }
    fn tell(&self) -> IoResult<u64> {fail!();}
}

impl std::io::Writer for NanoSocket {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        self.send(buf);
        return Ok(());
    }
    fn flush(&mut self) -> IoResult<()> {
      return Ok(());
    }
}


#[unsafe_destructor]
impl Drop for NanoSocket {
    fn drop(&mut self) {
        #![inline(never)]

        // close
        let rc = unsafe { nn_close (self.sock) };
        if rc != 0 {
            let msg = format!("nn_close({:?}) failed with errno: {:?} '{:?}'", self.sock, std::os::errno(), std::os::last_os_error());
            error!("{:s}", msg);
            fail!("{:s}", msg);
        }
    }
}



// ======================================================
// NanoMsg
//
//  It is not necessary to restrict the lifetime of the
//  NanoMsg to be a subset of the lifetime of the
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
pub struct NanoMsg {
    buf: *mut u8,
    bytes_stored_in_buf: u64,
    bytes_available: u64,
    cleanup: HowToCleanup,
}

impl NanoMsg {

    pub fn new() -> NanoMsg {
        let buf : *mut u8 = 0 as *mut u8;
        NanoMsg{buf: buf, bytes_stored_in_buf: 0, bytes_available: 0, cleanup: DoNothing }
    }

    pub fn len(&self) -> u64 {
        self.bytes_stored_in_buf
    }

    pub fn actual_msg_bytes_avail(&self) -> u64 {
        self.bytes_available
    }

    pub fn printbuf(&self) {
        println!("NanoMsg contains message of length {:?}: '{:s}'", self.bytes_stored_in_buf, self.copy_to_string());
    }

    /// Unwraps the NanoMsg.
    /// Any ownership of the message pointed to by buf is forgotten.
    /// Since we take self by value, no further access is possible.
    pub unsafe fn unwrap(self) -> *mut u8 {
        let mut msg = self;
        msg.cleanup = DoNothing;
        msg.buf
    }

    /// recv_any_size allows nanomsg to do zero-copy optimizations
    pub fn recv_any_size(&mut self, sock: c_int, flags: c_int) -> Result<u64, NanoErr>{
        #![inline(never)]

        match self.cleanup {
            DoNothing => (),
            Free => self.cleanup(),
            FreeMsg => self.cleanup()
        }

        unsafe {
            self.bytes_stored_in_buf = nn_recv (sock,  transmute(&mut self.buf), NN_MSG, flags) as u64;
        }
        self.bytes_available = self.bytes_stored_in_buf;

        if self.bytes_stored_in_buf < 0 {
            debug!("nn_recv failed with errno: {:?} '{:?}'", std::os::errno(), std::os::last_os_error());
            return Err( NanoErr{rc: std::os::errno() as i32, errstr: last_os_error() } );
        }

        self.cleanup = FreeMsg;
        return Ok(self.bytes_stored_in_buf);
    }


    /// Use recv_no_more_than_maxlen() if we need our own copy anyway, but don't want to overflow our
    /// heap. The function will truncate any part of the message over maxlen. In general, prefer recv_any_size() above.
    pub fn recv_no_more_than_maxlen(&mut self, sock: c_int, maxlen: u64, flags: c_int) -> Result<u64, NanoErr> {
        #![inline(never)]

        match self.cleanup {
            DoNothing => (),
            Free => self.cleanup(),
            FreeMsg => self.cleanup()
        }

        unsafe {
            let ptr = malloc(maxlen as size_t) as *mut u8;
            assert!(!ptr.is_null());
            self.cleanup = Free;

            self.buf = ptr;
            self.bytes_available = nn_recv (sock,
                                           transmute(self.buf),
                                           maxlen,
                                           flags) as u64;

            if self.bytes_available < 0 {
                let errmsg = format!("recv_no_more_than_maxlen: nn_recv failed with errno: {:?} '{:?}'", std::os::errno(), std::os::last_os_error());
                warn!("{:s}", errmsg);
                return Err( NanoErr{rc: std::os::errno() as i32, errstr: last_os_error() } );
            }

            if self.bytes_available > maxlen {
                let errmsg = format!("recv_no_more_than_maxlen: message was longer ({:?} bytes) than we allocated space for ({:?} bytes)", self.bytes_available, maxlen);
                warn!("{:s}", errmsg);
            }

            self.bytes_stored_in_buf = min(maxlen, self.bytes_available);
            Ok(self.bytes_stored_in_buf)
        }
    }

    pub fn copy_to_string(&self) -> ~str {
        unsafe { std::str::raw::from_buf_len(self.buf as *u8, self.bytes_stored_in_buf as uint) }
    }

    pub fn cleanup(&self) {
        #![inline(never)]

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
                    // println!("*** FreeMsg Drop running.");

                    let x = intrinsics::init(); // dummy value to swap in
                    // moving the object out is needed to call the destructor
                    ptr::replace(self.buf, x);

                    let rc = nn_freemsg(self.buf as *mut c_void);
                    assert! (rc == 0);
                }
            }
        }

    }

}

#[unsafe_destructor]
impl Drop for NanoMsg {
    fn drop(&mut self) {
        #![inline(never)]
        // println!("starting Drop for NanoMsg, with style: {:?}", self.cleanup);
        self.cleanup();
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_test_msg_client_msg_server() {

        spawn(proc() {
            msgserver_test();
        });

        msgclient_test();
    }

    #[test]
    fn smoke_test_readerwriter() {
        let addr="tcp://127.0.0.1:1234";

        // server end:
        spawn(proc() {
            let mut sock = match NanoSocket::new(AF_SP, NN_PAIR) {
                Ok(s) => s,
                Err(_) => fail!("asdf")
            };

            sock.bind(addr);

            let mut buf = [0,0,0,0,0];
            sock.read(buf);
            assert!(buf[2] == 3);
        });


        // client end:
        let mut sock = match NanoSocket::new(AF_SP, NN_PAIR) {
            Ok(s) => s,
            Err(_) => fail!("asdf")
        };

        sock.connect(addr);
        sock.write([1,2,3,4]);
    }

    // basic test that NanoMsg and NanoSocket are working
    fn msgclient_test() {
        // make a NanoMsg to hold a received message
        let mut msg = NanoMsg::new();

        let address = "tcp://127.0.0.1:5439";

        { // sock lifetime

            // create and connect
            let sock = match NanoSocket::new(AF_SP, NN_PAIR) {
                Ok(s) => s,
                Err(e) => fail!("Failed with err:{:?} {:?}", e.rc, e.errstr)
            };

            sock.connect(address);

            // send
            let b = "WHY";
            sock.sendstr(b);

            // demonstrate NanoMsg::recv_any_size()
            match msg.recv_any_size(sock.sock, 0) {
                Ok(sz) => {
                    let m = msg.copy_to_string();
                    assert!(m.as_slice() == "LUV");
                },
                Err(e) => fail!("recv_any_size -> nn_recv failed with errno: {:?} '{:?}'", e.rc, e.errstr)
            }

            // it is okay to reuse msg (e.g. as below, or in a loop). NanoMsg will free any previous message before
            //  receiving a new one.
            // demonstrate NanoMsg::recv_no_more_than_maxlen()
            match msg.recv_no_more_than_maxlen(sock.sock, 2, 0) {
                Ok(sz) => {
                    let m = msg.copy_to_string();
                    assert!(m.as_slice() == "CA");
                },
                Err(e) => fail!("recv_no_more_than_maxlen -> nn_recv failed with errno: {:?} '{:?}'", e.rc, e.errstr)
            }
        } // end of socket lifetime
    }

    fn msgserver_test () {
        let mut msg = NanoMsg::new();
        let address = "tcp://127.0.0.1:5439";

        // create and connect
        let sock = match NanoSocket::new(AF_SP, NN_PAIR) {
            Ok(s) => s,
            Err(e) => fail!("Failed with err:{:?} {:?}", e.rc, e.errstr)
        };

        match sock.bind(address) {
            Ok(_) => {},
            Err(e) => fail!("Bind failed with err:{:?} {:?}", e.rc, e.errstr)
        }

        // receive
        match msg.recv_any_size(sock.sock, 0) {
            Ok(sz) => {
                let m = msg.copy_to_string();
                assert!(m.as_slice() == "WHY");
            },
            Err(e) => fail!("recv_any_size -> nn_recv failed with errno: {:?} '{:?}'", e.rc, e.errstr)
        }

        // send
        let b = "LUV";
        match sock.sendstr(b) {
            Ok(_) => {},
            Err(e) => fail!("send failed with err:{:?} {:?}", e.rc, e.errstr)
        }

        // send 2
        let b = "CAT";
        match sock.sendstr(b) {
            Ok(_) => {},
            Err(e) =>{
                fail!("send failed with err:{:?} {:?}", e.rc, e.errstr);
            }
        }
    }

    #[test]
    fn test_getsockopt() {
        let addr="tcp://127.0.0.1:8898";

        let mut sock = match NanoSocket::new(AF_SP, NN_PAIR) {
            Ok(s) => s,
            Err(_) => fail!("asdf")
        };

        sock.bind(addr);

        // Linger default is 1000ms
        let ret = match sock.getsockopt(NN_SOL_SOCKET, NN_LINGER) {
          Ok(s) => s,
          Err(e) => fail!("failed getsockopt(NN_SOL_SOCKET, NN_LINGER) with err:{:?} {:?}", e.rc, e.errstr)
        };

        assert!(ret == 1000);

        // SendBuffer default is 128kb (131072 bytes)
        let ret = match sock.getsockopt(NN_SOL_SOCKET, NN_SNDBUF) {
          Ok(s) => s,
          Err(e) => fail!("failed getsockopt(NN_SOL_SOCKET, NN_SNDBUF) with err:{:?} {:?}", e.rc, e.errstr)
        };

        assert!(ret == 131072);

        // ReceiveBuffer default is 128kb (131072 bytes)
        let ret = match sock.getsockopt(NN_SOL_SOCKET, NN_RCVBUF) {
          Ok(s) => s,
          Err(e) => fail!("failed getsockopt(NN_SOL_SOCKET, NN_RCVBUF) with err:{:?} {:?}", e.rc, e.errstr)
        };

        assert!(ret == 131072);

        // Send timeout default is -1 (unlimited)
        let ret = match sock.getsockopt(NN_SOL_SOCKET, NN_SNDTIMEO) {
          Ok(s) => s,
          Err(e) => fail!("failed getsockopt(NN_SOL_SOCKET, NN_SNDTIMEO) with err:{:?} {:?}", e.rc, e.errstr)
        };

        assert!(ret == -1);


    }
}
