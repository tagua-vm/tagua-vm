# Tagua VM [![build status](https://api.travis-ci.org/tagua-vm/tagua-vm.svg)](https://travis-ci.org/tagua-vm/tagua-vm)

Tagua VM is an experimental [PHP](http://php.net/) Virtual Machine written with
[the Rust language](https://www.rust-lang.org/) and [the LLVM Compiler
Infrastructure](http://llvm.org/).

## Goals

The PHP language currently has two majors virtual machines (VM): [Zend
Engine](https://en.wikipedia.org/wiki/Zend_Engine) and
[HHVM](http://hhvm.com/).  Zend Engine is the original VM, it is mainly written
in C and counts hundreds of contributors. HHVM is mainly written in C++ and
also counts hundreds of contributors. HHVM, by being more recent, has a more
state-of-the-art approach and offers features like Just-In-Time compilation.

However, both VM are written in unsafe languages, where segmentation faults,
data races, memory corruptions etc. are very frequent and are severe
errors/vulnerabilities.

Tagua VM has a different approach.

1. It is written in Rust: A language that guarantees memory safety, threads
   without data races (by using the move semantics, data ownership, borrowing,
   lifetime…), zero-cost abstractions, minimal runtime and, as a bonus,
   efficient C bindings,
2. It relies on LLVM for the compiler backend: A solid, state-of-the-art,
   research, widely used modular and reusable compiler and toolchains
   technologies.

Instead of re-developing our own algorithms to get a better VM, LLVM will be
used. It natively provides a typed [Intermediate Representation (IR)
language](http://llvm.org/docs/LangRef.html), kind of an “opcode” (to match
the classical PHP vocabulary). Because this IR language is typed, it forces us
to have advanced analysis about PHP types, but this is another topic.

### Safety first

The legend says that PHP powers more than 80% of the Web applications. The two
biggest websites in terms of traffic, namely
[Wikipedia](https://wikipedia.org/) and [Facebook](https://facebook.com/), are
written in PHP. Thus, this is extremely important to have a safe VM to run
these applications.

Since the old days of Computer Science, numerous bugs and vulnerabilities in OS
(like Linux or BSD), in libraries (like Gclibc), in major programs (like Bash or
X.org), have been found, simply due to the lack of memory and type safety.
Intrinsically, Rust enforces safety statically, hence removing most of the
memory vulnerabilities like segmentation faults or data races.

LLVM, as for it, is written in C++. Rust provides efficient C bindings and can
check the safety of these bindings as most as possible. Rust stops checking when
encountering an `unsafe` block. We commit to land the smallest unsafe surfaces
as much as possible and abstracting the data from Rust to LLVM in order to help
Rust ensuring the safety.

### High quality

The quality of a project can be defined in various ways. Here is what we mean
when speaking about quality.

1. Documentation: Always up-to-date, detailed as much as possible, both for API
   and user documentations.
2. Unit tests: Each functions, each structures, each traits is unit tested.
   No code lands without a unit test.
3. Integration tests: Tagua VM is both a library and a binary; the library part
   also has an integration test suite.
4. Continuous Integration: Each set of commits must compile and must not
   introduce a regression on all build targets.

## Roadmap

* [x] Parser with strong types and zero-copy,
* [x] Minimal LLVM safe bindings:
  * [x] Context, module, builder, value…,
  * [x] Engine: Use [MCJIT](http://llvm.org/docs/MCJITDesignAndImplementation.html) for the JIT engine,
  * [x] Native types: Bindings from Rust to LLVM,
* [x] Minimal complete chains (from PHP file to a real execution),
* [ ] Type inference algorithms to reduce memory consumption,
* (not defined yet)

## Installing

To install Tagua VM, you must have Rust (see [the Rust installation
page](https://www.rust-lang.org/downloads.html)) and LLVM (see [the LLVM
installation page](http://llvm.org/releases/download.html)) installed. This is
important to have `llvm-config` available in the path. This is also important
to have `cargo` available in the path too.
[Cargo](http://doc.crates.io/guide.html) is the Rust package manager.

To build a release version:

```sh
$ cargo build --release
$ ./target/release/tvm --help
```

To build a development version:

```sh
$ cargo build
$ ./target/debug/tvm --help
```

## Using docker

If you don't want to install Rust and LLVM on your machine you can use docker:
it provides everything you will need to build, test and run Tagua VM.

First, you will need to build the docker image:

```sh
$ docker build -t tagua-vm .
```

You will then be able to run a container from this image:

```sh
$ docker run --rm -it --name tagua-vm-dev -v `pwd`:/source` bash
```

You are now inside a fresh container. To see if everything is fine, you can
start the test suite:

```sh
$ cargo test
```

## Contributing

Do whatever you want. Just respect the license and the other contributors. Your
favorite tool is going to be:

```sh
$ cargo test
```

to run all the test suites (unit test suites, integration test suites and documentation test suites).

### カンバン

In order to get an overview of what need to be done, what is in progress and
what has been recently done, [a kanban board is
available](https://waffle.io/tagua-vm/tagua-vm).

## Documentation

The documentation is not online yet. To generate it locally, please, run the following command:

```sh
$ cargo doc
$ open target/doc/tagua_vm/index.html
```

## License

Tagua VM is under the New BSD License (BSD-3-Clause):

```
                                New BSD License



Copyright © 2016-2016, Ivan Enderlin.
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of the Hoa nor the names of its contributors may be
      used to endorse or promote products derived from this software without
      specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS AND CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
```
