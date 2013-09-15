use std::libc::*;
use std::c_str::*;
use nanomsg::*;
mod nanomsg;

#[fixed_stack_segment]
fn main ()
{
    let mut msg = NanoMsg::new();
    let SOCKET_ADDRESS = "tcp://127.0.0.1:5555";
    printfln!("server binding to '%?'", SOCKET_ADDRESS);

    // create and connect
    let sockret = NanoSocket::new(AF_SP, NN_PAIR);
    let sock : NanoSocket;
    match sockret {
        Ok(s) => {
            sock = s;
        },
        Err(e) =>{
            fail!(fmt!("Failed with err:%? %?", e.rc, e.errstr));
        }
    }
    
    sockret = sock.bind(SOCKET_ADDRESS);
    match sockret {
        Ok => {},
        Err(e) =>{
            fail!(fmt!("Bind failed with err:%? %?", e.rc, e.errstr));
        }
    }

    // get a buffer for receive
    let mut v = 0 as *mut u8;
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
    printfln!("server: I received a %d byte long msg: '%s'", recv_msg_size as int, msg.to_str());

    // send
    let b = "LUV";
    let buf = b.to_c_str();
    let rc : c_int = buf.with_ref(|b| unsafe { nn_send (sc, b as *std::libc::types::common::c95::c_void, 3, 0) });
    printfln!("server: I sent '%s'", b);
    
    assert!(rc >= 0); // errno_assert
    assert!(rc == 3); // nn_assert


    // send 2
    let b = "CAT";
    let buf = b.to_c_str();
    let rc : c_int = buf.with_ref(|b| unsafe { nn_send (sc, b as *std::libc::types::common::c95::c_void, 3, 0) });
    printfln!("server: 2nd send, I sent '%s'", b);
    
    assert!(rc >= 0); // errno_assert
    assert!(rc == 3); // nn_assert


    // dealloc
    let rc = unsafe { nn_freemsg(v as *mut std::libc::types::common::c95::c_void) };
    assert! (rc == 0);
    
    // close
    let rc = unsafe { nn_close (sc) };
    assert!(rc == 0); // errno_assert

}


use nanomsg::*;
mod nanomsg;


fn main ()
{
    // make a NanoMsg to hold a received message
    let mut msg = NanoMsg::new();

    let SOCKET_ADDRESS = "tcp://127.0.0.1:5555";
    printfln!("client binding to '%?'", SOCKET_ADDRESS);

    // verify that msg lifetime can outlive the socket
    // from whence it came
    
    { // sock lifetime

        // create and connect
        let sockret = NanoSocket::new(AF_SP, NN_PAIR);
        let sock : NanoSocket;
        match sockret {
          Ok(s) => {
            sock = s;
          },
          Err(e) =>{
            fail!(fmt!("Failed with err:%? %?", e.rc, e.errstr));
          }
        }
        sock.connect(SOCKET_ADDRESS);
        
        // send
        let b = "WHY";
        sock.send(b);
        printfln!("client: I sent '%s'", b);
        

        // demonstrate NanoMsg::recv_any_size()
        let recd = msg.recv_any_size(sock.sock, 0);
        
        match(recd) {
            None => {
                fail!("recv_any_size -> nn_recv failed with errno: %? '%?'", std::os::errno(), std::os::last_os_error());
            },
            Some(sz) => {

                printfln!("actual_msg_size is %?", sz);
                
                let m = msg.copy_to_string();
                printfln!("client: I received a %d byte long msg: '%s', of which I have '%?' bytes in my buffer.", recd.unwrap() as int, m, msg.actual_msg_bytes_avail());

                // also available for debugging:
                // msg.printbuf();
                
            }
        }
    
    
       // it is okay to reuse msg (e.g. as below, or in a loop). NanoMsg will free any previous message before
       //  receiving a new one.
    
       // demonstrate NanoMsg::recv_no_more_than_maxlen()
       let recd = msg.recv_no_more_than_maxlen(sock.sock, 2, 0);
    
       match(recd) {
           None => {
               fail!("recv_any_size -> nn_recv failed with errno: %? '%?'", std::os::errno(), std::os::last_os_error());
           },
           Some(sz) => {
               
               printfln!("recv_no_more_than_maxlen got back this many bytes: %?", sz);
               
               let m = msg.copy_to_string();
               
               printfln!("client: I received a %d byte long msg: '%s', while there were '%?' bytes available from nanomsg.", recd.unwrap() as int, m, msg.actual_msg_bytes_avail());
               
               // also available for debugging:
               // msg.printbuf();
               
           }
       }
   } // end of socket lifetime

   print(fmt!("verify that message is still around: "));
   msg.printbuf();
} // end main


