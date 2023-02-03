FROM rust:1.65-alpine3.16

#RUN rustup toolchain install stable-x86_64-unknown-linux-gnu
#RUN rustup target add x86_64-unknown-linux-musl
#RUN rustup target install --toolchain stable-x86_64-unknown-linux-gnu  x86_64-unknown-linux-musl
#RUN apt-get install musl-tools

WORKDIR /usr/src/sagittarius-a

COPY . .

RUN rm -r -f ./target && cargo build --release --target x86_64-unknown-linux-musl

# RUN apt-get install cmake musl-tools clang libc++-dev build-essential autoconf libtool pkg-config 
