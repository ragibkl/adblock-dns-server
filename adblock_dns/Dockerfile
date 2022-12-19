# builder
FROM rust:alpine as builder
RUN apk add musl-dev
RUN cargo install --no-default-features --features rustls-tls adblock-list-compiler


# runner
FROM alpine:latest
RUN apk --update add bind gettext bash

COPY bind/. /etc/bind/.
COPY entrypoint.sh /entrypoint.sh

COPY --from=builder /usr/local/cargo/bin/ablc /usr/local/bin/ablc

EXPOSE 53

ENTRYPOINT /entrypoint.sh
