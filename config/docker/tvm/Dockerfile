FROM debian:stretch

ENV ARCH=x86_64-unknown-linux-gnu
ENV RUST_RELEASE=1.9.0
ENV CARGO_RELEASE=nightly

COPY Cargo.* /tmp/src/
COPY source/ /tmp/src/source/

RUN apt-get update -y && \
    apt-get install -y curl llvm-3.6 gcc libssl-dev libedit-dev && \
    ln -s /usr/lib/x86_64-linux-gnu/libstdc++.so.6 /usr/lib/x86_64-linux-gnu/libstdc++.so && \
    find /usr/bin -executable -iname llvm* | xargs -n1 -I file echo ln -s file file | sed s/-3.6$// | bash && \
    curl -sL https://static.rust-lang.org/dist/rust-$RUST_RELEASE-$ARCH.tar.gz | tar xvz -C /tmp && \
    /tmp/rust-$RUST_RELEASE-$ARCH/install.sh && \
    curl -sL https://static.rust-lang.org/cargo-dist/cargo-$CARGO_RELEASE-$ARCH.tar.gz | tar xvz -C /tmp && \
    /tmp/cargo-$CARGO_RELEASE-$ARCH/install.sh && \
    cd /tmp/src/ && cargo build && cp /tmp/src/target/debug/tvm /usr/bin/ && \
    apt-get remove --purge -y curl && \
    apt-get autoclean && apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

WORKDIR /source
VOLUME /source

ENTRYPOINT ["tvm"]
