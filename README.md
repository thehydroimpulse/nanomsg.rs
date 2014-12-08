# Rust Nanomsg [![Build Status](https://travis-ci.org/thehydroimpulse/nanomsg.rs.svg?branch=master)](https://travis-ci.org/thehydroimpulse/nanomsg.rs)

![Cargo 0.3.0](http://img.shields.io/badge/cargo-0.3.0-orange.svg?style=flat)
![MIT License](http://img.shields.io/npm/l/express.svg?style=flat)

Nanomsg is a modern messaging library that is the successor to ZeroMQ, written in C by Martin Sustrik and colleagues. The nanomsg library is licensed under MIT/X11 license. "nanomsg" is a trademark of 250bpm s.r.o.

- http://nanomsg.org/
- https://github.com/nanomsg/nanomsg

### Requirements

You'll need to have nanomsg installed beforehand.

## Getting Started

This library is Cargo supported! Simply add a new cargo dependency and
away you go!

```toml
[dependencies.nanomsg]
git = "https://github.com/thehydroimpulse/nanomsg.rs"
```

Now you can use the crate after you include it:

```rust
extern crate nanomsg;
```

## Creating a Socket (Push/Pull)

The basis of Nanomsg is a `Socket`. Each socket can be of a certain type. The type of a socket defines it's behaviour and limitations (such as only being able to send and not receive).

```rust
use nanomsg::{Socket, Protocol, NanoResult};

fn initialize() -> NanoResult<()> {
    let mut socket = try!(Socket::new(Protocol::Pull));
    let mut endpoint = try!(socket.bind("ipc:///tmp/pipeline.ipc"));

    loop {
        let msg = try!(socket.read_to_string());
        println!("We got a message: {}", msg.as_slice());
    }
}

fn main() {
    initialize();
}
```

As you can see, we bind a socket to an endpoint. These resources are destroyed correctly at the end of their lifetime, closing the bound socket.

We can also create a client to that socket. The `Pull` protocol is one that can only receive (push/pull), so we need the accompanying socket `Push` for this to work correctly (or to get a message in the end).

(For now, we'll create them as separate binaries because of some threading issues with nanomsg)

```rust
use nanomsg::{Socket, Protocol, NanoResult};

fn initialize() -> NanoResult<()> {
    let mut socket = try!(Socket::new(Protocol::Push));
    let mut endpoint = try!(socket.connect("ipc:///tmp/pipeline.ipc"));

    socket.write(b"message in a bottle");

    endpoint.shutdown();
}

fn main() {
    initialize();
}
```

## Contributors

(In arbitrary order):

* Jason E. Aten ([@glycerine](https://github.com/glycerine))
* David C. Bishop ([@dcbishop](https://github.com/dcbishop))
* Dennis Lawler ([@evenodder](https://github.com/evenodder))
* Daniel Fagnan ([@TheHydroImpulse](https://github.com/thehydroimpulse))
* Zachary Tong ([@polyfractal](https://github.com/polyfractal))
* Dan Burkert ([@danburkert](https://github.com/danburkert))
* Benoît Labaere ([@blabaere](https://github.com/blabaere))

## License

This project is under the same license as Rust. Dual MIT and Apache 2.

The MIT License (MIT)

* Copyright (c) 2013-2014 Jason E. Aten, Ph.D. [@glycerine](https://github.com/glycerine)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
