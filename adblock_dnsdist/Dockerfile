# dnstap
FROM golang:alpine as dnstap
RUN go install github.com/dnstap/golang-dnstap/dnstap@v0.4.0

# dnsdist
FROM alpine:latest
RUN apk add dnsdist certbot

COPY --from=dnstap /go/bin/dnstap /usr/bin/.
COPY dnsdist.template.conf /etc/dnsdist.template.conf
COPY entrypoint.sh /entrypoint.sh

EXPOSE 53/tcp 53/udp 80 443
ENTRYPOINT /entrypoint.sh
