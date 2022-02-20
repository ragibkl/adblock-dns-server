FROM alpine:latest

RUN apk add dnsdist certbot

COPY dnsdist.conf.template /etc/dnsdist.conf.template
COPY entrypoint.sh /entrypoint.sh

EXPOSE 53/tcp 53/udp 80 443
ENTRYPOINT /entrypoint.sh
