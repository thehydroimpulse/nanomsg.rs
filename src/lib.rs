#![crate_type = "lib"]
#![license = "MIT/ASL2"]
#![feature(globs, unsafe_destructor, phase)]

#[phase(plugin, link)] extern crate log;

extern crate libc;

extern crate libnanomsg;

mod result;

