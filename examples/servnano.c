#include <string.h>
#include <stdio.h>
#include <nanomsg/nn.h>
#include <nanomsg/pair.h>
#include <nanomsg/pubsub.h>
#include <nanomsg/tcp.h>
#include "./nano_err.h" // copy over /usr/cn/nanomsg/src/utils/err.h locally.


// ==================================
// ==================================
// servnano.c : micro tcp server example
// ==================================
// ==================================

// gcc -g -o servnano servnano.c -lnanomsg

#define SOCKET_ADDRESS "tcp://127.0.0.1:5555"

int main ()
{
  int rc;
  int sb;
  int sc;
  int i;
  char buf [4];
  int opt;
  size_t sz;

  // server

  sb = nn_socket (AF_SP, NN_PAIR);
  errno_assert (sb >= 0);

  // bind
  rc = nn_bind (sb, SOCKET_ADDRESS);
  errno_assert (rc > 0);

  // receive
  bzero(buf, 4);
  rc = nn_recv(sb, buf, sizeof(buf), 0);
  errno_assert(rc >= 0);
  nn_assert(rc == 3);

  printf("server: I received: '%s'\n", buf);

  // send
  memcpy(buf, "LUV\0", 4); // trailing null for later printf
  rc = nn_send (sb, buf, 3, 0);
  printf("server: I sent: '%s'\n", buf);

  errno_assert (rc >= 0);
  nn_assert (rc == 3);
   
  // close
  rc = nn_close (sb);
  errno_assert (rc == 0);

  return 0;
}


