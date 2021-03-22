FROM ragibkl/adblock_dns:base

COPY data data

RUN compiler && mv data/output.d/blacklist.zone /etc/bind/blacklist.zone
