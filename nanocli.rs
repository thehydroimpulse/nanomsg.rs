use std::libc::*;
use std::c_str::*;
use std::ptr;
use std::unstable::intrinsics;
use std::cast::transmute;
use std::option::Option;
use std::num::*;

use nanomsg::*;
mod nanomsg;

enum HowToCleanup {
  Free,
  Call_nn_freemsg,
  DoNothing
}

// a wrapper around the message returned by nn_recv
pub struct NanoMsg {
    buf: *mut u8,
    size: u64,
    priv cleanup: HowToCleanup,
}


impl NanoMsg {

    pub fn new() -> NanoMsg {
        let buf : *mut u8 = 0 as *mut u8;
        NanoMsg{buf: buf, size: 0, cleanup: DoNothing }
    }

    pub fn len(&self) -> u64 {
        self.size
    }

    pub fn printbuf(&self) {
        printfln!("NanoMsg contains message of length %?: '%s'", self.size, self.copy_to_string());
    }

    /// Unwraps the NanoMsg.
    /// Any ownership of the message pointed to by buf is forgotten.
    pub unsafe fn unwrap(self) -> *mut u8 {
        printfln!("we should never get here!!!!");
        assert!(false);
        let mut msg = self;
        msg.cleanup = DoNothing;
        msg.buf
    }

    pub fn recv_any_size(&mut self, sock: c_int, flags: c_int) -> Option<u64> {
        #[fixed_stack_segment];
        #[inline(never)];

        unsafe { 
            self.size = nn_recv (sock,  transmute(&mut self.buf), NN_MSG, flags) as u64;
        }

        if (self.size < 0) {
            printfln!("nn_recv failed with errno: %? '%?'", std::os::errno(), std::os::last_os_error());
            return None;
        } else {
            return Some(self.size);
        }
    }


    // truncates any part of the message over maxlen
    pub fn recv_no_more_than_maxlen(&mut self, sock: c_int, maxlen: u64, flags: c_int) -> Option<u64> {
        #[fixed_stack_segment];
        #[inline(never)];


        unsafe { 
            self.cleanup = Free;
            let ptr = malloc(maxlen as size_t) as *mut u8;
            assert!(!ptr::is_null(ptr));

            self.buf = ptr;
            let actual_sz_of_msg = nn_recv (sock, 
                                 transmute(self.buf),
                                 maxlen, 
                                 flags) as u64;

            if (actual_sz_of_msg < 0) {
                printfln!("recv_no_more_than_maxlen: nn_recv failed with errno: %? '%?'", std::os::errno(), std::os::last_os_error());
                return None;
            }

            if (actual_sz_of_msg > maxlen) {
                printfln!("recv_no_more_than_maxlen: message was longer (%? bytes) than we allocated space for (%? bytes)", actual_sz_of_msg, maxlen);
                warn!("recv_no_more_than_maxlen: message was longer (%? bytes) than we allocated space for (%? bytes)", actual_sz_of_msg, maxlen);
            }

            self.size = min(maxlen, actual_sz_of_msg);            
            Some(self.size)
        }
    }

    pub fn copy_to_string(&self) -> ~str {
        printfln!("copy to string sees size: '%?'", self.size);
        printfln!("copy to string sees buf : '%?'", self.buf as *u8);
        unsafe { std::str::raw::from_buf_len(self.buf as *u8, self.size as uint) }
    }

}

#[unsafe_destructor]
impl Drop for NanoMsg {
    fn drop(&self) {
        #[fixed_stack_segment];
        #[inline(never)];
        printfln!("starting Drop for NanoMsg");

        if (std::ptr::is_null(self.buf)) { return; }

        match self.cleanup {
            DoNothing => (),
            Free => {
                unsafe {
                    // see example code: http://static.rust-lang.org/doc/tutorial-ffi.html

                    let x = intrinsics::init(); // dummy value to swap in
                    // moving the object out is needed to call the destructor
                    ptr::replace_ptr(self.buf, x);
                    free(self.buf as *c_void)
                }
            },

            Call_nn_freemsg => {
                unsafe {
                    let x = intrinsics::init(); // dummy value to swap in
                    // moving the object out is needed to call the destructor
                    ptr::replace_ptr(self.buf, x);
                    
                    let rc = nn_freemsg(self.buf as *mut c_void);
                    assert! (rc == 0);
                }
            }
            
        }
    }
}



#[fixed_stack_segment]
fn main ()
{

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

    {
        let mut msg = NanoMsg::new();
        
        // receive
        let recd = msg.recv_any_size(sc, 0);
        
        match(recd) {
            None => {
                fail!("recv_any_size -> nn_recv failed with errno: %? '%?'", std::os::errno(), std::os::last_os_error());
            },
            Some(sz) => {

                printfln!("actual_msg_size is %?", sz);
                
                let m = msg.copy_to_string();
                
                // this to_str() call will only work for utf8, but for now that's enough
                // to let us verify we have the connection going.
                printfln!("client: I received a %d byte long msg: '%s', of which I have '%?' bytes in my buffer.", recd.unwrap() as int, m, msg.len());

                // msg.printbuf();
                
            }
        }


        let recd = msg.recv_no_more_than_maxlen(sc, 2, 0);

        match(recd) {
            None => {
                fail!("recv_any_size -> nn_recv failed with errno: %? '%?'", std::os::errno(), std::os::last_os_error());
            },
            Some(sz) => {

                printfln!("actual_msg_size is %?", sz);
                
                let m = msg.copy_to_string();
                
                printfln!("client: I received a %d byte long msg: '%s', of which I have '%?' bytes in my buffer.", recd.unwrap() as int, m, msg.len());

                // msg.printbuf();
                
            }
        }


    }
    
    // close
    let rc = unsafe { nn_close (sc) };
    assert!(rc == 0); // errno_assert

}
