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
 quickly.


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

While the api file in nanomsg.rs is all that you really need to link in an unsafe way, 
the examples are not code to be emulated!

The client and server sample code in nanocli.rs and nanoserv.rs uses lots of unsafe calls directly in to the C code.  This is just my learning the rust language. Be aware (and beware) that this interface is *not* what a client API to a foreign library should provide in rust.  In other words, it doesn't wrap the C library in a safe API.  In Rust, like C++, the usual practice is to provide a 'safe' interface to an unsafe library, where the constructor, reference getting, and destructor patterns create memory and resource safety. The result of a safe interface is that clients need not use unsafe{} blocks.

Nonetheless I'm publishing this as it stands a) as a starting point; and b) to get feedback; and c) because the nanomsg.rs binding itself may be quite useful to others, as an example of how wrapping and rust-bingen work. (It wasn't obvious to me at all when starting.)

Status:  The binding appears to work just fine, although do note
	 that both rust and nanomsg are in active development.
	 The test programs (nanoserv.rs, nanocli.rs) demonstrate
	 the bindings in action; they create an nn_socket and send
	 and receive messages over the wire. Valgrind reports
	 that the rust versions leak 40 bytes each, which is
	 of concern.

Other protocols beyond nn_socket: they work in the C side,
         rust examples/contributions welcome. Feel fre to fork 
         this repo and send me pull requests with improvements.
