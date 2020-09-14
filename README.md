# Nanomsg 

![Cargo 0.7.2](http://img.shields.io/badge/cargo-0.7.2-orange.svg?style=flat)
![MIT License](http://img.shields.io/npm/l/express.svg?style=flat)
[![Build Status](https://travis-ci.org/thehydroimpulse/nanomsg.rs.svg?branch=master)](https://travis-ci.org/thehydroimpulse/nanomsg.rs) 
[![Build status](https://ci.appveyor.com/api/projects/status/hwfjigfwyomc56u1?svg=true)](https://ci.appveyor.com/project/thehydroimpulse/nanomsg-rs)


[Documentation](http://thehydroimpulse.github.io/nanomsg.rs/nanomsg)

Nanomsg is a modern messaging library that is the successor to ZeroMQ, written in C by Martin Sustrik and colleagues. The nanomsg library is licensed under MIT/X11 license. "nanomsg" is a trademark of 250bpm s.r.o.

- http://nanomsg.org/
- https://github.com/nanomsg/nanomsg

### Requirements

* Nanomsg 1.1.4

Installing nanomsg:

```
make deps
```

## Installation

```toml
[dependencies]
nanomsg = "0.7.2"
```

Simply import the crate to use it:

```rust
use nanomsg;
```

## Creating a Socket

The basis of Nanomsg is a `Socket`. Each socket can be of a certain type. The type of a socket defines it's behaviour and limitations (such as only being able to send and not receive).

```rust
use nanomsg::{Socket, Protocol, Error};

/// Creating a new `Pull` socket type. Pull sockets can only receive messages
/// from a `Push` socket type.
fn create_socket() -> Result<(), Error> {
    let mut socket = Socket::new(Protocol::Pull)?;
    Ok(())
}
```

Now, each socket that is created can be bound to *multiple* endpoints. Each binding can return an error, so
we'll take advantage of the `?` (try) operator.

```rust
use nanomsg::{Socket, Protocol, Error};

/// Creating a new `Pull` socket type. Pull sockets can only receive messages
/// from a `Push` socket type.
fn create_socket() -> Result<(), Error> {
    let mut socket = Socket::new(Protocol::Pull)?;
    
    // Create a new endpoint bound to the following protocol string. This returns
    // a new `Endpoint` that lives at-most the lifetime of the original socket.
    let mut endpoint = socket.bind("ipc:///tmp/pipeline.ipc")?;

    Ok(())
}
```

The socket is ready to be used now!

Because this is a `Pull` socket, we'll implement reading any messages we receive.

```rust
// ... After the endpoint we created, we'll start reading some data.
let mut msg = String::new();
loop {
    socket.read_to_string(&mut msg)?;
    println!("We got a message: {}", &*msg);
    msg.clear();
}
// ...
```

That's awesome! But... we have no packets being sent to the socket, so we'll read nothing. To fix this, let's implement the accompanying pair `Push` socket.

```rust
use nanomsg::{Socket, Protocol, Error};

fn pusher() -> Result<(), Error> {
    let mut socket = Socket::new(Protocol::Push)?;
    let mut endpoint = socket.connect("ipc:///tmp/pipeline.ipc")?;

    socket.write(b"message in a bottle");

    endpoint.shutdown();
    Ok(())
}
```

## Contributors

(In arbitrary order):

* Daniel Fagnan ([@TheHydroImpulse](https://github.com/thehydroimpulse))
* Jason E. Aten ([@glycerine](https://github.com/glycerine))
* David C. Bishop ([@dcbishop](https://github.com/dcbishop))
* Dennis Lawler ([@evenodder](https://github.com/evenodder))
* Zachary Tong ([@polyfractal](https://github.com/polyfractal))
* Dan Burkert ([@danburkert](https://github.com/danburkert))
* Benoît Labaere ([@blabaere](https://github.com/blabaere))
* Kevin Butler ([@Ryman](https://github.com/Ryman))
* Andrew ([@GGist](https://github.com/GGist))
* Chip Collier ([@photex](https://github.com/photex))
* Zeke Foppa ([@bfops](https://github.com/bfops))
* Philippe Delrieu ([@musitdev](https://github.com/musitdev))
* Daniel Kozlowski ([@dkhenry](https://github.com/dkhenry))
* Vinzent Steinberg ([@vks](https://github.com/vks))
* Paul Woolcock ([@pwoolcoc](https://github.com/pwoolcoc))
* Gabriel Martinez ([@mystal](https://github.com/mystal))
* wdv4758h ([@wdv4758h](https://github.com/wdv4758h))
* Alexander Morozov ([@alexandermorozov](https://github.com/alexandermorozov))
* Jan S ([@jan-schreib](https://github.com/jan-schreib))
* Thayne McCombs ([@tmccombs](https://github.com/tmccombs))
* Anders Bennehag ([@PureW](https://github.com/PureW))
* Peter Parkanyi ([@rsdy](https://github.com/rsdy))
* Jacek ([@forgerpl](https://github.com/forgerpl))
* Gal Schlezinger ([@Schniz](https://github.com/Schniz))
* Gilad Naaman ([@Gilnaa](https://github.com/Gilnaa))

## License

The MIT License (MIT)

* Copyright (c) 2013-2014 Jason E. Aten, Ph.D. [@glycerine](https://github.com/glycerine)
* Copyright (c) 2014 Daniel Fagnan [@thehydroimpulse](https://github.com/thehydroimpulse)
* Copyright (c) 2015-2018 Benoît Labaere [@blabaere](https://github.com/blabaere)

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
