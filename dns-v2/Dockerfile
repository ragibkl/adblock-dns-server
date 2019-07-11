FROM ubuntu:14.04

RUN apt-get update && apt-get install -y bind9

COPY templates /templates
COPY replace.sh /replace.sh
COPY entrypoint.sh /entrypoint.sh

EXPOSE 53

ENTRYPOINT /entrypoint.sh
