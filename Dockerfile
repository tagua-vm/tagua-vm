FROM debian:jessie

ENV ARCH=x86_64-unknown-linux-gnu
ENV RUST_RELEASE=1.7.0
ENV CARGO_RELEASE=nightly

RUN echo "deb http://llvm.org/apt/jessie/ llvm-toolchain-jessie main" > /etc/apt/sources.list.d/llvm.list && \
    apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 15CF4D18AF4F7421 && \
    apt-get update && \
    apt-get install -y curl llvm-3.8 vim gcc libssl-dev libedit-dev && \
    find /usr/bin -executable -iname llvm* | xargs -n1 -I file echo ln -s file file | sed s/-3.8$// | bash

RUN curl -sL https://static.rust-lang.org/dist/rust-$RUST_RELEASE-$ARCH.tar.gz | tar xvz -C /tmp && \
    /tmp/rust-$RUST_RELEASE-$ARCH/install.sh && \
    rm -rf /tmp/tmp/rust-$RUST_RELEASE-$ARCH

RUN curl -sL https://static.rust-lang.org/cargo-dist/cargo-$CARGO_RELEASE-$ARCH.tar.gz | tar xvz -C /tmp && \
    /tmp/cargo-$CARGO_RELEASE-$ARCH/install.sh && \
    rm -rf /tmp/cargo-$CARGO_RELEASE-$ARCH

RUN apt-get remove --purge -y curl && \
    apt-get autoclean && apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

VOLUME /source
WORKDIR /source
