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
    
    let ret = sock.bind(SOCKET_ADDRESS);
    match ret {
        Ok(_) => {},
        Err(e) =>{
            fail!(fmt!("Bind failed with err:%? %?", e.rc, e.errstr));
        }
    }

    // receive
    let recd = msg.recv_any_size(sock.sock, 0);
    match(recd) {
        Err(e) => {
            fail!("recv_any_size -> nn_recv failed with errno: %? '%?'", e.rc, e.errstr);
        },
        Ok(sz) => {
            printfln!("actual_msg_size is %?", sz);
            
            let m = msg.copy_to_string();
            printfln!("client: I received a %? byte long msg: '%s', of which I have '%?' bytes in my buffer.", sz, m, msg.actual_msg_bytes_avail());

            // also available for debugging:
            // msg.printbuf();
        }
    }

    // send
    let b = "LUV";
    let ret = sock.send(b);
    match ret {
        Ok(_) => {},
        Err(e) =>{
            fail!(fmt!("send failed with err:%? %?", e.rc, e.errstr));
        }
    }
    printfln!("server: I sent '%s'", b);

    // send 2
    let b = "CAT";
    let ret = sock.send(b);
    match ret {
        Ok(_) => {},
        Err(e) =>{
            fail!(fmt!("send failed with err:%? %?", e.rc, e.errstr));
        }
    }
    printfln!("server: 2nd send, I sent '%s'", b);
}



