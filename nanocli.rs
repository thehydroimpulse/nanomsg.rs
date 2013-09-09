use std::libc::*;
use std::c_str::*;
use std::ptr;
use std::unstable::intrinsics;

use nanomsg::*;
mod nanomsg;

// a wrapper around the message returned by nn_recv
pub struct NanoMsg {
    buf: *mut u8,
    size: u64,
    priv abuf: *mut *mut u8,
    priv owns_buffer_: bool,
}


impl NanoMsg {

    pub fn new() -> NanoMsg {
        let mut buf : *mut u8 = 0 as *mut u8;
        let abuf = &mut buf;
        NanoMsg{buf: buf, abuf: abuf, size: 0, owns_buffer_: false }
    }

    /// Unwraps the NanoMsg.
    /// Any ownership of the message pointed to by buf is forgotten.
    pub unsafe fn unwrap(self) -> *mut u8 {
        let mut msg = self;
        msg.owns_buffer_ = false;
        msg.buf
    }


    pub fn recv_NN_MSG(&mut self, sock: c_int, flags: c_int) -> u64 {
        #[fixed_stack_segment];
        #[inline(never)];

        unsafe { 
            self.size = nn_recv (sock,  self.abuf as *mut std::libc::types::common::c95::c_void, NN_MSG, flags) as u64;
        }
        self.size
    }

    // truncates any part of the message over len
    pub fn recv_fixed_size(&mut self, sock: c_int, len: u64, flags: c_int) -> u64 {
        #[fixed_stack_segment];
        #[inline(never)];

        self.size = unsafe { 
            nn_recv (sock, 
                     self.abuf as *mut std::libc::types::common::c95::c_void, 
                     len, 
                     flags) as u64
        };
        self.size
    }

    pub fn copy_to_string(&self) -> ~str {
        unsafe { std::str::raw::from_buf_len(self.buf as *u8, self.size as uint) }
    }

    // the 'r lifetime results in the same semantics as `&mut *x` with ~T
    pub fn borrow_mut<'r>(&'r mut self) -> ~str {
        unsafe { std::str::raw::from_buf_len(self.buf as *u8, self.size as uint) }
    }


}

#[unsafe_destructor]
impl Drop for NanoMsg {
    fn drop(&self) {
        #[fixed_stack_segment];
        #[inline(never)];

        unsafe {
            let x = intrinsics::init(); // dummy value to swap in
            // moving the object out is needed to call the destructor
            ptr::replace_ptr(self.buf, x);
            let rc = nn_freemsg(self.buf as *mut c_void);
            assert! (rc == 0);
        }
    }
}




// useful examples of code from bjz (thanks!):
/*
 https://github.com/bjz/glfw-rs#example-code
 https://github.com/bjz/glfw-rs/blob/master/src/glfw/lib.rs#L645
 https://github.com/bjz/glfw-rs/blob/master/src/glfw/lib.rs#L1069
*/

// if you want to be sure you are running on the main thread,
// do this:
#[start]
#[fixed_stack_segment]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    // Run on the main thread
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

// TODO: figure out how to make a safe interface that
//       wraps all these unsafe calls.


#[fixed_stack_segment]
fn main ()
{
    cli2();
}

#[fixed_stack_segment]
fn cli1() {

    let SOCKET_ADDRESS = "tcp://127.0.0.1:5555";
    printfln!("client binding to '%?'", SOCKET_ADDRESS);

    let sc : c_int = unsafe { nn_socket (AF_SP, NN_PAIR) };
    printfln!("nn_socket returned: %?", sc);

    assert!(sc >= 0);
    
    // connect
    let addr = SOCKET_ADDRESS.to_c_str();
    let rc : c_int = addr.with_ref(|a| unsafe { nn_connect (sc, a) });
    assert!(rc > 0);
    
    // send
    let b = "WHY";
    let buf = b.to_c_str();
    let rc : c_int = buf.with_ref(|b| unsafe { nn_send (sc, b as *std::libc::c_void, 3, 0) });
    printfln!("client: I sent '%s'", b);
    
    assert!(rc >= 0); // errno_assert
    assert!(rc == 3); // nn_assert

    // get a pointer, v, that will point to
    // the buffer that receive fills in for us.

    let mut v: *mut u8 = std::ptr::mut_null();
    //    let mut v = 0 as *mut u8;    // this also works to get v started.

    let x: *mut *mut u8 = &mut v;

    // receive
    let recv_msg_size = unsafe { nn_recv (sc, x as *mut std::libc::types::common::c95::c_void, NN_MSG, 0) };

    if (rc < 0) {
        printfln!("nn_recv failed with errno: %? '%?'", std::os::errno(), std::os::last_os_error());
    }

    assert! (rc >= 0); // errno_assert

    let msg = unsafe { std::str::raw::from_buf_len(v as *u8, recv_msg_size as uint) };

    // this to_str() call will only work for utf8, but for now that's enough
    // to let us verify we have the connection going.
    printfln!("client: I received a %d byte long msg: '%s'", recv_msg_size as int, msg.to_str());

    // dealloc
    let rc = unsafe { nn_freemsg(v as *mut std::libc::types::common::c95::c_void) };
    assert! (rc == 0);
    
    // close
    let rc = unsafe { nn_close (sc) };
    assert!(rc == 0); // errno_assert

}



#[fixed_stack_segment]
fn cli2() {

    let SOCKET_ADDRESS = "tcp://127.0.0.1:5555";
    printfln!("client binding to '%?'", SOCKET_ADDRESS);

    let sc : c_int = unsafe { nn_socket (AF_SP, NN_PAIR) };
    printfln!("nn_socket returned: %?", sc);

    assert!(sc >= 0);
    
    // connect
    let addr = SOCKET_ADDRESS.to_c_str();
    let rc : c_int = addr.with_ref(|a| unsafe { nn_connect (sc, a) });
    assert!(rc > 0);
    
    // send
    let b = "WHY";
    let buf = b.to_c_str();
    let rc : c_int = buf.with_ref(|b| unsafe { nn_send (sc, b as *std::libc::c_void, 3, 0) });
    printfln!("client: I sent '%s'", b);
    
    assert!(rc >= 0); // errno_assert
    assert!(rc == 3); // nn_assert

    // get a pointer, v, that will point to
    // the buffer that receive fills in for us.

    let mut msg = NanoMsg::new();

    // receive
    let recv_msg_size = msg.recv_NN_MSG(sc, 0);

    if (recv_msg_size < 0) {
        printfln!("nn_recv failed with errno: %? '%?'", std::os::errno(), std::os::last_os_error());
    }

    assert! (rc >= 0); // errno_assert

    let m = msg.copy_to_string();

    // this to_str() call will only work for utf8, but for now that's enough
    // to let us verify we have the connection going.
    printfln!("client: I received a %d byte long msg: '%s'", recv_msg_size as int, m);

    
    // close
    let rc = unsafe { nn_close (sc) };
    assert!(rc == 0); // errno_assert

}
