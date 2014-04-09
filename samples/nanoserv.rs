extern crate nanomsg;
use std::io::Writer;
use nanomsg::AF_SP;
use nanomsg::NN_PAIR;
use nanomsg::NanoSocket;


fn main ()
{
    let SOCKET_ADDRESS = "tcp://127.0.0.1:5555";
    println!("server binding to '{:?}'", SOCKET_ADDRESS);

    // create and connect
    let sockret = NanoSocket::new(AF_SP, NN_PAIR);
    let mut sock = match sockret {
        Ok(s) => s,
        Err(e) => fail!("Failed with err:{:?} {:?}", e.rc, e.errstr)
    };
    
    match sock.bind(SOCKET_ADDRESS) {
        Ok(_) => {},
        Err(e) =>{
            fail!("Bind failed with err:{:?} {:?}", e.rc, e.errstr);
        }
    }

    // receive
    let recd = match sock.recv() {
        Ok(v) => {
            println!("actual_msg_size is {:?}", v.len());
            
            match std::str::from_utf8(v) {
              Some(msg) => println!("server: I received a {} byte long msg: '{:s}'", v.len(), msg),
              None() => println!("server: I received a {} byte long msg but it was None'", v.len()),
            }
        },
        Err(e) => fail!("sock.recv -> failed with errno: {:?} '{:?}'", e.rc, e.errstr)
    };

    let b = "LUV";
    // send
    match sock.send(b.as_bytes()) {
        Ok(_) => {},
        Err(e) => fail!("send failed with err:{:?} {:?}", e.rc, e.errstr)
    }

    println!("server: I sent '{:s}'", b);

    // send 2, using Writer interface
    let b = "CAT";
    sock.write(b.as_bytes());

    println!("server: 2nd send, I sent '{:s}'", b);
}



