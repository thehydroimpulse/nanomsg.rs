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
                printfln!("client: I received a %d byte long msg: '%s', of which I have '%?' bytes in my buffer.", recd.unwrap() as int, m, msg.actual_msg_bytes_avail());

                // msg.printbuf();
                
            }
        }


        let recd = msg.recv_no_more_than_maxlen(sc, 2, 0);

        match(recd) {
            None => {
                fail!("recv_any_size -> nn_recv failed with errno: %? '%?'", std::os::errno(), std::os::last_os_error());
            },
            Some(sz) => {

                printfln!("recv_no_more_than_maxlen got back this many bytes: %?", sz);
                
                let m = msg.copy_to_string();
                
                printfln!("client: I received a %? byte long msg: '%?', of which I have '%?' bytes in my buffer.", msg.len(), m, recd.unwrap() as int);

                // msg.printbuf();
                
            }
        }


    }
    
    // close
    let rc = unsafe { nn_close (sc) };
    assert!(rc == 0); // errno_assert

}
