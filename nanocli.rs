
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
            Err(e) => {
                fail!("recv_any_size -> nn_recv failed with errno: %? '%?'", e.rc, e.errstr);
            },
            Ok(sz) => {

                printfln!("actual_msg_size is %?", sz);
                
                let m = msg.copy_to_string();
                printfln!("client: I received a %? byte long msg: '%s', of which I have '%?' bytes in my buffer.",  sz, m, msg.actual_msg_bytes_avail());

                // also available for debugging:
                // msg.printbuf();
                
            }
        }
    
    
       // it is okay to reuse msg (e.g. as below, or in a loop). NanoMsg will free any previous message before
       //  receiving a new one.
    
       // demonstrate NanoMsg::recv_no_more_than_maxlen()
       let recd = msg.recv_no_more_than_maxlen(sock.sock, 2, 0);
    
       match(recd) {
           Err(e) => {
               fail!("recv_any_size -> nn_recv failed with errno: %? '%?'", e.rc, e.errstr);
           },
           Ok(sz) => {
               
               printfln!("recv_no_more_than_maxlen got back this many bytes: %?", sz);
               
               let m = msg.copy_to_string();
               
               printfln!("client: I received a %? byte long msg: '%s', while there were '%?' bytes available from nanomsg.", sz, m, msg.actual_msg_bytes_avail());
               
               // also available for debugging:
               // msg.printbuf();
               
           }
       }
   } // end of socket lifetime

   print(fmt!("verify that message is still around: "));
   msg.printbuf();
} // end main

