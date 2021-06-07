FROM ekidd/rust-musl-builder as builder

COPY compiler/Cargo.toml Cargo.toml
COPY compiler/Cargo.lock Cargo.lock
COPY compiler/src src

RUN cargo build --release


FROM alpine:latest
RUN apk --update add bind

COPY dns_base/bind/. /etc/bind/.
COPY dns_base/entrypoint.sh /entrypoint.sh

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/compiler /usr/local/bin/compiler

EXPOSE 53

ENTRYPOINT /entrypoint.sh
