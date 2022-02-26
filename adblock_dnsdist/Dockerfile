FROM alpine:latest

RUN apk add dnsdist certbot go
RUN go install github.com/dnstap/golang-dnstap/dnstap@latest

COPY dnsdist.template.conf /etc/dnsdist.template.conf
COPY entrypoint.sh /entrypoint.sh

EXPOSE 53/tcp 53/udp 80 443
ENTRYPOINT /entrypoint.sh
