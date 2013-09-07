rust-nanomsg
============

Summary: nanomsg bindings for rust

rust: http://www.rust-lang.org/
nanomsg: http://nanomsg.org/

Rust is a modern langauge from Mozilla Research. It has  support for 
 writing embedded applications that are memory safe and simultaneously
 do not suffer garbage-collection pauses.

nanomsg is a modern messaging library that is the 
 successor to ZeroMQ, written in C by Martin Sustrick and colleagues.
 The nanomsg library is licensed under MIT/X11 license. "nanomsg" 
 is a trademark of 250bpm s.r.o.

These rust-nanogen bindings were initiated using an automated bindings
 generator called rust-bindgen from Jyun-Yan You (repository:
 https://github.com/crabtw/rust-bindgen ) and then hand edited to
 include necessary pub static constants extracted manually. The
 later process involved using the cpp -dD flag to extract #defines,
 and rewriting them as "pub static MYCONST: int = 1;" statements.
