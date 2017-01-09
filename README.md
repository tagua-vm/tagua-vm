# Tagua VM

[![Build status](https://api.travis-ci.org/tagua-vm/tagua-vm.svg?branch=master)](https://travis-ci.org/tagua-vm/tagua-vm)
[![Chat on Freenode](https://img.shields.io/badge/chat-on_%23taguavm-ff0066.svg)](https://webchat.freenode.net/?channels=#taguavm)
[![Chat on Gitter](https://img.shields.io/badge/chat-on_gitter-ff0066.svg)](https://gitter.im/tagua-vm/tagua-vm)

[![Join the chat at https://gitter.im/tagua-vm/tagua-vm](https://badges.gitter.im/tagua-vm/tagua-vm.svg)](https://gitter.im/tagua-vm/tagua-vm?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Tagua VM is an experimental [PHP][1] Virtual Machine that
guarantees safety and quality by removing large classes of vulnerabilities
thanks to [the Rust language][2] and [the LLVM Compiler
Infrastructure][3].

## Introduction

PHP is an extremely popular programming language. On 2015, [PHP was used by
more than 80% of all websites][4]. However, almost [500 known severe
vulnerabilities have been recorded][5], whose almost [50 with a high CVE
score][6]. This is inherent to any popular language but this is dangerous.

The goal of this project is to provide a PHP VM that guarantees safety and
quality by removing large classes of vulnerabilities. This will be achieved by
using appropriated tools like Rust and LLVM. Rust is a remarkable language that
brings strong guarantees about memory safety. This is also a fast language that
competes well with C. It can also talk to C very easily. LLVM is a famous
compiler infrastructure that brings modernity, state-of-the-art algorithms,
performance, developer tool chains…

This project will solve three problems at once:

  1. **Providing safety and high quality by removing large classes of
     vulnerabilities and thus avoid the cost of dramatic bugs**,
  2. **Providing modernity, new developer experience and state-of-the-art
     algorithms so performance**,
  3. **Providing a set of libraries that will compose the VM and that can be
     reused outside of the project (like the parser, analysers, extensions
     etc.)**.

## Why PHP is critical?

[PHP][1] is a popular programming language. Today, it powers a large part of
the softwares we daily use on Internet. To list a few: Wikipedia, Facebook,
Baidu, Yahoo, or Pinterest, but also softwares you can install for your own
purposes, such as: Wordpress (blog and website), Drupal (CMS), Joomla (CMS),
Magento (commerce), Shopify (commerce), Moodle (learning), phpBB (forum)….  On
2015, PHP was used as the server-side programming language on more than [80% of
all websites][4].

Due to its unique position in the Internet land, a single vulnerability could
have a huge impact.

  * Economical: Imagine a shop; A bug in the check out and products can no
    longer be sold, or prices can be wrong. This is critical for both a small or a
    big business. Note that this could be done by a malicious person,
  * Privacy: Imagine a social network with hundreds of thousands enthusiasms; A
    bug could leak private or personal information,
  * Organisational: Imagine a music festival; A bug in the event software can
    cause the cancellation of several months of works,
  * Any other: Memory corruption, segmentation fault, data races… all these
    class of bugs can be critical at so much levels.

PHP VMs have recorded almost [500 known vulnerabilities][5], whose almost
[50 vulnerabilities with a CVE score greater or equal to 9 over 10][6]. Many of
them and the most dangerous are about memory corruptions [[7], [8], [9]] or
errors in parsers [[10], [11], [12], [13], [14]]. The implications of these
vulnerabilities are for instance remote code execution or Denial Of Service, two
vectors that have important impact on a whole infrastructure.

This situation is real for any programming language (like [Python][15] or
[Java][16]). Nevertheless, the criticality of a vulnerability is hardly linked
to the popularity of the language. In the case of PHP, a single vulnerability
can be dangerous in so many fashions. However, this is not the fault of the
language itself: All the listed vulnerabilities are due to the VMs.

## What is the big plan?

Currently, PHP has two major virtual machines (VM): [Zend Engine][17] and
[HHVM][18]. Zend Engine is the original VM, it is mainly written in C and
counts hundreds of contributors. HHVM is mainly written in C++ and also counts
hundreds of contributors. HHVM, by being more recent, has a more
state-of-the-art approach and offers features like Just-In-Time compilation.

However, both VM are written in unsafe languages, where segmentation faults,
data races, memory corruptions etc. are very frequent and are severe
errors/vulnerabilities, as presented in the previous section.

Tagua VM has a different approach.

  * It is written in [Rust][2]: A language that guarantees memory safety,
    threads without data races (by using the move semantics, data ownership,
    borrowing, lifetime…), zero-cost abstractions, minimal runtime and, as
    a bonus, efficient C bindings, in addition to being as fast as C,
  * It relies on [LLVM][3] for the compiler backend: A solid,
    state-of-the-art, research, widely used modular and reusable compiler and
    toolchains technologies.

The class of vulnerabilities and class of bugs mentioned earlier are almost
removed in the Rust language. This is part of its guarantees. It does not avoid
the need of complete test suites and security audits though.

### Safety first

Due to the popularity of PHP, this is extremely important to have a safe VM to
run these applications.

Since the old days of Computer Science, numerous bugs and vulnerabilities in OS
(like Linux or BSD), in libraries (like Gclibc), in major programs (like Bash,
X.org or PHP VMs), have been found, simply due to the lack of memory and
type safety. Intrinsically, Rust enforces safety statically, hence removing most
of the memory vulnerabilities like segmentation faults or data races.

### High quality

The quality of a project can be defined in various ways. Here is what we mean
when speaking about quality.

  * Documentation: Always up-to-date, detailed as much as possible, both for
    API and user documentations.
  * Unit tests: Each function, each structure, each trait is unit tested. No
    code lands without a unit test.
  * Integration tests: Tagua VM is both a set of libraries and a binary; The
    libraries part also has an integration test suites.
  * Continuous Integration: Each set of commits must compile and must not
    introduce a regression on all build targets.

### Performance

The term “performance” must be defined. By saying “performance” we mean: Speed
and memory efficiency. While speed is not the top priority, memory is. It is
part of safety. When safety is ensured and quality is high enough to detect most
of the regressions, we can safely patch the VM to get better performances if and
when necessary.

Obviously, we commit to use the state-of-the-art algorithms and structures to
ensure excellent performances.

## Roadmap

(Under rewriting).

## Installing

To install Tagua VM, you must have Rust (see [the Rust installation page][19])
and LLVM (see [the LLVM installation page][20]) installed. This is important to
have `llvm-config` and `cargo` available in the path. [Cargo][21] is the Rust
package manager.

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

### Using Docker

If installing Rust and LLVM on your machine is too much, Docker might be an
alternative: It provides everything needed to build, test and run Tagua VM.

First, build the Docker image:

```sh
$ docker build -t tagua-vm-dev .
```

Now, it is possible to run a container from this image:

```sh
$ docker run --rm -it -v $(pwd):/source tagua-vm-dev
```

If this command succeeds, you are inside a fresh container. To see if
everything is fine, you can start the test suite:

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

### カンバン ([Kanban](https://en.wikipedia.org/wiki/Kanban))

In order to get an overview of what need to be done, what is in progress and
what has been recently done, [a kanban board is
available](https://waffle.io/tagua-vm/tagua-vm).

## Documentation and help

The documentation is automatically uploaded online at the following address:
https://tagua-vm.github.io/tagua-vm.

To generate it locally, please, run the following command:

```sh
$ cargo doc --open
```

To get help on IRC, please join the official [`#taguavm` channel on
Freenode](https://webchat.freenode.net/?channels=#taguavm). Alternatively, there
is a [mirrored room on Gitter](https://gitter.im/tagua-vm/tagua-vm).

## Libraries

Tagua VM is designed as a set of libraries that can work outside of the
project. So far, the following libraries are living outside of the project:

  * [`libtagua_parser`][22], Safe, fast and memory efficient PHP parser
    (lexical and syntactic analysers).

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


[1]: http://php.net/
[2]: https://www.rust-lang.org/
[3]: http://llvm.org/
[4]: https://w3techs.com/technologies/details/pl-php/all/all
[5]: https://www.cvedetails.com/vendor/74/PHP.html
[6]: https://www.cvedetails.com/vulnerability-list.php?vendor_id=74&product_id=&version_id=&page=1&hasexp=0&opdos=0&opec=0&opov=0&opcsrf=0&opgpriv=0&opsqli=0&opxss=0&opdirt=0&opmemc=0&ophttprs=0&opbyp=0&opfileinc=0&opginf=0&cvssscoremin=9&cvssscoremax=0&year=0&month=0&cweid=0&order=1&trc=495&sha=3a14a3e67be8aa88a16a1018e06ffe4e0d940af5
[7]: https://www.cvedetails.com/cve/CVE-2016-2554/
[8]: https://www.cvedetails.com/cve/CVE-2015-8880/
[9]: https://www.cvedetails.com/cve/CVE-2015-5589/
[10]: https://www.cvedetails.com/cve/CVE-2015-8617/
[11]: https://www.cvedetails.com/cve/CVE-2015-4642/
[12]: https://www.cvedetails.com/cve/CVE-2015-4601/
[13]: https://www.cvedetails.com/cve/CVE-2016-4544/
[14]: https://www.cvedetails.com/cve/CVE-2016-4539/
[15]: https://www.cvedetails.com/vulnerability-list/vendor_id-10210/Python.html
[16]: http://www.cvedetails.com/vulnerability-list.php?vendor_id=5&product_id=1526&version_id=&page=1&hasexp=0&opdos=0&opec=0&opov=0&opcsrf=0&opgpriv=0&opsqli=0&opxss=0&opdirt=0&opmemc=0&ophttprs=0&opbyp=0&opfileinc=0&opginf=0&cvssscoremin=9&cvssscoremax=0&year=0&month=0&cweid=0&order=1&trc=435&sha=0050c3562eb6901e183f2b2b636c9769579a0fb8
[17]: http://zend.com/
[18]: http://hhvm.com/
[19]: https://www.rust-lang.org/downloads.html
[20]: http://llvm.org/releases/download.html
[21]: http://doc.crates.io/guide.html
[22]: https://github.com/tagua-vm/parser
