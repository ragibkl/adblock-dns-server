FROM ragibkl/adblock_dns:base

COPY data data

RUN compiler
RUN cp data/output.d/blacklist.zone /etc/bind/blacklist.zone
