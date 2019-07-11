#!/usr/bin/env bash

TIMESTAMP=$(date +"%Y%m%d%H")
FQDN="${FQDN:-localhost.localdomain}"
IPV4="${IPV4:-127.0.0.1}"
IPV6="${IPV6:-::1}"

FORWARDER_1="${FORWARDER_1:-8.8.8.8}"
FORWARDER_2="${FORWARDER_2:-8.8.4.4}"

ZONE_NAME="${ZONE_NAME:-blacklist}"
ZONE_PATH="${ZONE_PATH:-/etc/bind/blacklist.zone}"

mkdir -p config

cat templates/named.conf.local | \
    sed s,'/%ZONE_NAME%/',"${ZONE_NAME}",g | \
    sed s,'/%ZONE_PATH%/',"${ZONE_PATH}",g > \
    config/named.conf.local

cat templates/named.conf.options | \
    sed s,'/%FORWARDER_1%/',"${FORWARDER_1}",g | \
    sed s,'/%FORWARDER_2%/',"${FORWARDER_2}",g | \
    sed s,'/%ZONE_NAME%/',"${ZONE_NAME}",g | \
    sed s,'/%ZONE_PATH%/',"${ZONE_PATH}",g > \
    config/named.conf.options

cat templates/null.zone.file | \
    sed s,'/%TIMESTAMP%/',"${TIMESTAMP}",g | \
    sed s,'/%FQDN%/',"${FQDN}",g | \
    sed s,'/%IPV4%/',"${IPV4}",g | \
    sed s,'/%IPV6%/',"${IPV6}",g > \
    config/null.zone.file

cat templates/blacklist.zone | \
    sed s,'/%IPV4%/',"${IPV4}",g | \
    sed s,'/%IPV6%/',"${IPV6}",g > \
    config/blacklist.zone
