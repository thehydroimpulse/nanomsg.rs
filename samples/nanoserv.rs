extern crate nanomsg;
use std::io::Writer;
use nanomsg::AF_SP;
use nanomsg::NN_PAIR;
use nanomsg::NanoSocket;


fn main ()
{
    let socket_address = "tcp://127.0.0.1:5555";
    println!("server binding to '{:?}'", socket_address);

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
    
    let ret = sock.bind(socket_address);
    match ret {
        Ok(_) => {},
        Err(e) =>{
            fail!("Bind failed with err:{:?} {:?}", e.rc, e.errstr);
        }
    }

    // receive
    let recd = sock.recv();
    match recd {
        Err(e) => {
            fail!("sock.recv -> failed with errno: {:?} '{:?}'", e.rc, e.errstr);
        },
        Ok(v) => {
            println!("actual_msg_size is {:?}", v.len());
            
            let m = std::str::from_utf8(v);
            match m {
              Some(msg) => println!("server: I received a {} byte long msg: '{:s}'", v.len(), msg),
              None => println!("server: I received a {} byte long msg but it was None'", v.len()),
            }
        }
    }

    // send
    let b = "LUV";
    let ret = sock.send(b.as_bytes());
    match ret {
        Ok(_) => {},
        Err(e) =>{
            fail!("send failed with err:{:?} {:?}", e.rc, e.errstr);
        }
    }
    println!("server: I sent '{:s}'", b);

    // send 2, using Writer interface
    let b = "CAT";
    sock.write(b.as_bytes());

    println!("server: 2nd send, I sent '{:s}'", b);
}



