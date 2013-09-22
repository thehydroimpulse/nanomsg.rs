#include <string.h>
#include <stdio.h>
#include <nanomsg/nn.h>
#include <nanomsg/pair.h>
#include <nanomsg/pubsub.h>
#include <nanomsg/tcp.h>
#include "./nano_err.h" // copy over /usr/cn/nanomsg/src/utils/err.h locally.

//include "/usr/cn/nanomsg/src/utils/err.c"
//include "/usr/cn/nanomsg/src/utils/sleep.c"

// ==================================
// ==================================
// client.c : running on Windows
// ==================================
// ==================================

// gcc -g -o clinano clinano.c -lnanomsg


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
  char msg[256];
  char* pRecvd = 0;

  // client, running on windows

  sc = nn_socket (AF_SP, NN_PAIR);
  errno_assert (sc >= 0);

  // connect
  rc = nn_connect (sc, SOCKET_ADDRESS);
  errno_assert (rc > 0);

  // send
  memcpy(buf, "WHY\0", 4);
  rc = nn_send (sc, buf, 3, 0);
  printf("client: I sent '%s'\n", buf);
  errno_assert (rc >= 0);
  nn_assert (rc == 3);

  // receive
  rc = nn_recv (sc, &pRecvd, NN_MSG, 0);
  errno_assert (rc >= 0);
  nn_assert (rc == 3);

  sprintf(msg, "client: I received: '%s'\n\0", buf);
  printf(msg);

  // free
  rc = nn_freemsg(pRecvd);
  errno_assert (rc == 0);

  // close
  rc = nn_close (sc);
  errno_assert (rc == 0);

  return 0;
}

