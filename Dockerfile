FROM debian:jessie

ENV ARCH=x86_64-unknown-linux-gnu
ENV RUST_RELEASE=1.9.0
ENV LLVM_RELEASE=3.9
ENV CARGO_RELEASE=nightly

RUN echo "deb http://llvm.org/apt/jessie/ llvm-toolchain-jessie main" > /etc/apt/sources.list.d/llvm.list && \
    apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 15CF4D18AF4F7421 && \
    apt-get update && \
    apt-get install -y curl llvm-$LLVM_RELEASE vim gcc libssl-dev libedit-dev libstdc++-4.9-dev && \
    find /usr/bin -executable -iname llvm* | xargs -n1 -I file echo ln -s file file | sed s/-$LLVM_RELEASE$// | bash

RUN curl -sL https://static.rust-lang.org/dist/rust-$RUST_RELEASE-$ARCH.tar.gz | tar xvz -C /tmp && \
    /tmp/rust-$RUST_RELEASE-$ARCH/install.sh && \
    rm -rf /tmp/rust-$RUST_RELEASE-$ARCH

RUN curl -sL https://static.rust-lang.org/cargo-dist/cargo-$CARGO_RELEASE-$ARCH.tar.gz | tar xvz -C /tmp && \
    /tmp/cargo-$CARGO_RELEASE-$ARCH/install.sh && \
    rm -rf /tmp/cargo-$CARGO_RELEASE-$ARCH

RUN apt-get remove --purge -y curl && \
    apt-get autoclean && apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

VOLUME /source
WORKDIR /source
