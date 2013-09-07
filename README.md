rust-nanomsg
============

Summary: nanomsg bindings for rust

rust: http://www.rust-lang.org/
nanomsg: http://nanomsg.org/

Rust is a modern langauge from Mozilla Research. It has  support for 
 writing embedded applications that are memory safe and simultaneously
 do not suffer garbage-collection pauses.

nanomsg is a modern messaging library that is the 
 successor to ZeroMQ, written in C by Martin Sustrik and colleagues.
 The nanomsg library is licensed under MIT/X11 license. "nanomsg" 
 is a trademark of 250bpm s.r.o.

These rust-nanogen bindings were initiated using an automated bindings
 generator called rust-bindgen from Jyun-Yan You (repository:
 https://github.com/crabtw/rust-bindgen ) and then hand edited to
 include necessary pub static constants extracted manually. The
 later process involved using the cpp -dD flag to extract #defines,
 and rewriting them as "pub static MYCONST: int = 1;" statements.


Status:

  in development. Currently the test client program, nanocli.rs
  will not compile. Why not?  I suspect I'm not using rust ffi
  correctly, but it could also be a compiler bug.

  Scenario:
```
    $rust run nanocli.rs
    nanocli.rs:34:21: 34:59 error: mismatched types: expected `i32` but found `()` (expected i32 but found ())
    nanocli.rs:34     let sc : c_int = unsafe { nn_socket (AF_SP, NN_PAIR); };
                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

when the declaration of nn_socket (in nanomsg.rs) clearly states 
that it returns a c_int:

```
pub fn nn_socket(domain: c_int, protocol: c_int) -> c_int;
```
So the mystery/question is: why does rustc think that nn_socket returns void ()?

