
// The mod statement creates a module. You can define it inline or 
// load it from file:
//
//     mod foo;
//
// is equivalent to
//
//     mod foo { /* content of foo.rs */ }
//

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

    let sc : c_int = unsafe { nn_socket (AF_SP, NN_PAIR); };
    printfln!("nn_socket returned: %?", sc);
/*
  errno_assert (sc >= 0);

  // connect
  rc = nn_connect (sc, SOCKET_ADDRESS);
  errno_assert (rc > 0);

  // send
  let buf = "WHY";
  rc = nn_send (sc, buf, 3, 0);
  printf("client: I sent '%s'\n", buf);
  errno_assert (rc >= 0);
  nn_assert (rc == 3);

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
