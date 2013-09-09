use std::libc::*;
use std::c_str::*;
use nanomsg::*;
mod nanomsg;

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
