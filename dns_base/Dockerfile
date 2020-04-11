FROM alpine:latest
RUN apk --update add bind

COPY etc/bind/. /etc/bind/.
COPY templates /templates

COPY replace.sh /replace.sh
COPY entrypoint.sh /entrypoint.sh

EXPOSE 53

ENTRYPOINT /entrypoint.sh
