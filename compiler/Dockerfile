FROM ekidd/rust-musl-builder as builder

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src

RUN cargo build --release

FROM alpine:latest

WORKDIR /
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/compiler /usr/local/bin/compiler
CMD ["compiler"]
