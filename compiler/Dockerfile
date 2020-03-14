FROM rust as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src

RUN cargo install --path .

FROM rust:slim
COPY --from=builder /usr/local/cargo/bin/compiler /usr/local/bin/compiler

WORKDIR /

CMD ["compiler"]
