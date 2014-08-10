extern crate debug;
extern crate nanomsg;
use std::io::{Reader,Writer};
use nanomsg::AF_SP;
use nanomsg::NN_PAIR;
use nanomsg::NanoSocket;


fn main() {
    let socket_address = "tcp://127.0.0.1:5555";
    println!("client connecting to '{:s}'", socket_address);

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
                fail!("Failed with err:{:?} {:?}", e.rc, e.errstr);
            }
        }
        let ret = sock.connect(socket_address);
        match ret {
            Ok(_) => {},
            Err(e) =>{
                fail!("Failed with err:{:?} {:?}", e.rc, e.errstr);
            }
        }

        // send
        let b = "WHY";
        sock.write(b.as_bytes()).unwrap();
        println!("client: I sent '{:s}'", b);

        // demonstrante NanoMsgStream, NanoMsgReader, NanoMsgWriter.
        let mut buf = [0, .. 100];
        let res = sock.read(buf);

        match res {
            Err(..) => {
                fail!("read failed!");
            },
            Ok(sz) => {
                println!("read returned: {:?}", sz);

                let m = std::str::from_utf8(buf);
                match m {
                    Some(msg) => println!("client: I received a {:?} byte long msg: '{:s}', of which I have '{:?}' bytes in my buffer.",  sz, msg, buf.len()),
                    None => println!("client: I received a {:?} byte long msg but it was NONE, I have '{:?}' bytes in my buffer.",  sz, buf.len()),
                }

                // also available for debugging:
                // msg.printbuf();
            }
        }

        // demonstrate read with limited buffer.
        let mut buf = [0, ..2];
        let recd = sock.read(buf);

        match recd {
            Err(..) => {
                fail!("read failed!");
            },
            Ok(sz) => {

                println!("read got back this many bytes: {:?}", sz);

                let m = std::str::from_utf8(buf);

                match m {
                    Some(msg) => println!("client: I received a {:?} byte long msg: '{:s}'", sz, msg),
                    None => println!("client: I received a {:?} byte long msg but it was None'", sz),
                }

                // also available for debugging:
                // msg.printbuf();
            }
        }
    } // end of socket lifetime
} // end main
