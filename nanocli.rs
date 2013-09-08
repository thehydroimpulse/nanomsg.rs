
// The mod statement creates a module. You can define it inline or 
// load it from file:
//
//     mod foo;
//
// is equivalent to
//
//     mod foo { /* content of foo.rs */ }
//


use std::libc::*;
use std::c_str::*;
use nanomsg::*;
mod nanomsg;



#[fixed_stack_segment]
fn main ()
{
    let SOCKET_ADDRESS = "tcp://127.0.0.1:5555";
    printfln!("client binding to '%?'", SOCKET_ADDRESS);
/*
  c_int rc;
  c_int sb;
  c_int sc;
  c_int i;
  char buf [4];
  c_int opt;
  size_t sz;
  char msg[256];
*/

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
    
    /*
    
    
    // receive
    rc = nn_recv (sc, buf, sizeof (buf), 0);
    errno_assert (rc >= 0);
    nn_assert (rc == 3);
    
    printfln!("client: I received: '%s'\n", buf);
    
    // close
    rc = nn_close (sc);
    errno_assert (rc == 0);
    */

}
