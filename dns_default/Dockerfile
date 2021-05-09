FROM ragibkl/adblock_dns:base

COPY data /data

RUN compiler -f /data/configuration.yaml -o /etc/bind/blacklist.zone /
