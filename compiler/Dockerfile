FROM rust as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src

RUN cargo install --path .

FROM ubuntu:18.04

RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/compiler /usr/local/bin/compiler

WORKDIR /

CMD ["compiler"]
