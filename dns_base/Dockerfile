FROM ragibkl/adblock_compiler:rust as compiler

FROM alpine:latest
RUN apk --update add bind

COPY etc/bind/. /etc/bind/.
COPY templates /templates

COPY replace.sh /replace.sh
COPY entrypoint.sh /entrypoint.sh

COPY --from=compiler /usr/local/bin/compiler /usr/local/bin/compiler

EXPOSE 53

ENTRYPOINT /entrypoint.sh
