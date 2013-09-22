extern mod nanomsg;
extern mod std;
use nanomsg::*;
use std::rt::io::{Reader,Writer};


fn main ()
{
    let SOCKET_ADDRESS = "tcp://127.0.0.1:5555";
    printfln!("client connecting to '%s'", SOCKET_ADDRESS);

    // verify that msg lifetime can outlive the socket
    // from whence it came
    
    { // sock lifetime

        // create and connect
        let sockret = NanoSocket::new(AF_SP, NN_PAIR);
        let mut sock : NanoSocket;
        match sockret {
          Ok(s) => {
            sock = s;
          },
          Err(e) =>{
            fail!(fmt!("Failed with err:%? %?", e.rc, e.errstr));
          }
        }
        let ret = sock.connect(SOCKET_ADDRESS);
        match ret {
          Ok(_) => {},
          Err(e) =>{
            fail!(fmt!("Failed with err:%? %?", e.rc, e.errstr));
          }
        }
        
        // send
        let b = "WHY";
        sock.write(b.as_bytes());
        printfln!("client: I sent '%s'", b);
        
        // demonstrante NanoMsgStream, NanoMsgReader, NanoMsgWriter.
        let mut buf = [0, .. 100];
        let res = sock.read(buf);
        
        match(res) {
            None => {
                fail!("read failed!");
            },
            Some(sz) => {

                printfln!("read returned: %?", sz);
                
                let m = std::str::from_utf8(buf);
                printfln!("client: I received a %? byte long msg: '%s', of which I have '%?' bytes in my buffer.",  sz, m, buf.len());

                // also available for debugging:
                // msg.printbuf();
                
            }
        }
    
       // demonstrate read with limited buffer.
       let mut buf = [0, ..2];
       let recd = sock.read(buf);
    
        match(recd) {
            None => {
               fail!("read failed!");
            },
            Some(sz) => {
               
                printfln!("read got back this many bytes: %?", sz);
               
                let m = std::str::from_utf8(buf);
               
                printfln!("client: I received a %? byte long msg: '%s'", sz, m);
               
                // also available for debugging:
                // msg.printbuf();
               
            }
        }
    } // end of socket lifetime

} // end main

