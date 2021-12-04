FROM rust:latest

RUN apt-get update && apt-get upgrade && apt-get install musl-tools pkg-config libssl-dev build-essential -y

RUN rustup target add x86_64-unknown-linux-musl

RUN export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/

RUN export OPENSSL_INCLUDE_DIR=/usr/include/openssl

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl
