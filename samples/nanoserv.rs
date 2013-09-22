extern mod nanomsg;
use std::str::*;
use nanomsg::*;
use std::rt::io::Writer;


#[fixed_stack_segment]
fn main ()
{
    let SOCKET_ADDRESS = "tcp://127.0.0.1:5555";
    printfln!("server binding to '%?'", SOCKET_ADDRESS);

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
    
    let ret = sock.bind(SOCKET_ADDRESS);
    match ret {
        Ok(_) => {},
        Err(e) =>{
            fail!(fmt!("Bind failed with err:%? %?", e.rc, e.errstr));
        }
    }

    // receive
    let recd = sock.recv();
    match(recd) {
        Err(e) => {
            fail!("sock.recv -> failed with errno: %? '%?'", e.rc, e.errstr);
        },
        Ok(v) => {
            printfln!("actual_msg_size is %?", v.len());
            
            let m = from_utf8(v);
            printfln!("client: I received a %? byte long msg: '%s'", v.len(), m);
        }
    }

    // send
    let b = "LUV";
    let ret = sock.send(b.as_bytes());
    match ret {
        Ok(_) => {},
        Err(e) =>{
            fail!(fmt!("send failed with err:%? %?", e.rc, e.errstr));
        }
    }
    printfln!("server: I sent '%s'", b);

    // send 2, using Writer interface
    let b = "CAT";
    sock.write(b.as_bytes());

    printfln!("server: 2nd send, I sent '%s'", b);
}



