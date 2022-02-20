FROM alpine:latest

RUN apk add dnsdist certbot

COPY dnsdist.template.conf /etc/dnsdist.template.conf
COPY entrypoint.sh /entrypoint.sh

EXPOSE 53/tcp 53/udp 80 443
ENTRYPOINT /entrypoint.sh
