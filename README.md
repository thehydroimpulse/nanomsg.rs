# Rust Nanomsg [![Build Status](https://travis-ci.org/thehydroimpulse/nanomsg.rs.svg?branch=master)](https://travis-ci.org/thehydroimpulse/nanomsg.rs)

## Nanomsg

nanomsg is a modern messaging library that is the successor to ZeroMQ, written in C by Martin Sustrik and colleagues. The nanomsg library is licensed under MIT/X11 license. "nanomsg" is a trademark of 250bpm s.r.o.

- http://nanomsg.org/
- https://github.com/nanomsg/nanomsg

## Requirements

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

use nanomsg::Nanomsg;
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
