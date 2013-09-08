rust-nanomsg
============

Summary: nanomsg bindings for rust

rust
----- 
Rust is a modern langauge from Mozilla Research. It has  support for 
 writing embedded applications that are memory safe and simultaneously
 do not suffer garbage-collection pauses. license: dual MIT / Apache 2.

 You'll want the github MASTER branch of rust to do anything useful
 and up to date. The project has strong velocity, so it is evolving
 quickly. The rust version used here was:

       rustc 0.8-pre (124eb21 2013-09-06 23:35:57 -0700)

- http://www.rust-lang.org/
- https://github.com/mozilla/rust

nanomsg
-------
nanomsg is a modern messaging library that is the 
 successor to ZeroMQ, written in C by Martin Sustrik and colleagues.
 The nanomsg library is licensed under MIT/X11 license. "nanomsg" 
 is a trademark of 250bpm s.r.o.  I'm using the HEAD of the

       * master 244540c changed location of the repo reflected in README

 branch for nanomsg.

- http://nanomsg.org/
- https://github.com/nanomsg/nanomsg

rust-nanomsg bindings
---------------------

These rust-nanogen bindings were initiated using an automated bindings
 generator called rust-bindgen from Jyun-Yan You (repository:
 https://github.com/crabtw/rust-bindgen ) and then hand edited to
 include necessary pub static constants extracted manually. The
 later process involved using the cpp -dD flag to extract #defines,
 and rewriting them as "pub static MYCONST: int = 1;" statements.

WARNING!
========

While the api file in nanomsg.rs is all that you really need to link in an unsafe way, 
the examples are not code to be emulated!

The client and server sample code (in nanocli.rs and nanoserv.rs) use lots of unsafe calls directly in to the C code.  This is just my learning the rust language. Be aware (and beware) that this interface is *not* what a client API to a foreign library should provide in rust.  In other words, it doesn't wrap the C library in a safe API.  

In Rust, like C++, the usual practice is to provide a 'safe' interface to an unsafe library, where the constructor, reference getting, and destructor patterns create memory and resource safety. The result of a safe interface is that clients need not use unsafe{} blocks.

Nonetheless I'm publishing this as it stands a) as a starting point; and b) to get feedback; and c) because the nanomsg.rs binding itself may be quite useful to others, as an example of how wrapping and rust-bindgen work. (It wasn't obvious to me at all when starting.)

Status:  
-------
The binding appears to work just fine, although do note
that both rust and nanomsg are in active development.
The test programs (nanoserv.rs, nanocli.rs) demonstrate
the bindings in action; they create an nn_socket and send
and receive messages over the wire. 

Valgrind status:
---------------

Valgrind reports
that the rust client (nanocli.rs) and server (nanoserv.rs)
leak 40 bytes each, which is of concern. We only transfered
4 bytes between client and server!  The same patterns in C code 
(included in this repo as clinano.c and servnano.c) do not leak 
at all under valgrind, so the leak has to be due to either my mis-use
of rust, or a leak in the rust-generated code or runtime.

Other protocols beyond nn_socket: 
---------------------------------

The other scalability protocols implemented in nanomsg work in the C side,
so rust ports/examples/contributions are welcome. Please feel free to fork 
this repo and send me pull requests with improvements. 
