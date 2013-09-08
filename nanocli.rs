use std::libc::*;
use std::c_str::*;
use nanomsg::*;
mod nanomsg;



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
    let rc : c_int = unsafe { nn_connect (sc, addr.unwrap() as *i8) };
    assert!(rc > 0);
    
    // send
    let b = "WHY";
    let buf = b.to_c_str();
    let rc : c_int = unsafe { nn_send (sc, buf.unwrap() as *std::libc::c_void, 3, 0) };
    printfln!("client: I sent '%s'", b);
    
    assert!(rc >= 0); // errno_assert
    assert!(rc == 3); // nn_assert

    // get a buffer for recevie
    let v : *mut c_void = unsafe { nn_allocmsg(16, 0) as *mut c_void };

    // receive
    let rc = unsafe { nn_recv (sc, v as *mut c_void, 3, 0) };
    assert! (rc >= 0); // errno_assert
    assert! (rc == 3); // nn_assert

    let msg = unsafe { std::str::raw::from_buf_len(v as *u8, 3) };
    printfln!("client: I received: '%s'\n", msg.to_str());
    
    // close
    let rc = unsafe { nn_close (sc) };
    assert!(rc == 0); // errno_assert

}

