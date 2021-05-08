#!/bin/sh

TIMESTAMP=$(date +"%Y%m%d%H")
FQDN="${FQDN:-localhost.localdomain}"
IPV4="${IPV4:-127.0.0.1}"
IPV6="${IPV6:-::1}"

FORWARDER_1="${FORWARDER_1:-8.8.8.8}"
FORWARDER_2="${FORWARDER_2:-8.8.4.4}"

cat /etc/bind/named.conf.template | \
    sed s,'/%FORWARDER_1%/',"${FORWARDER_1}",g | \
    sed s,'/%FORWARDER_2%/',"${FORWARDER_2}",g > \
    /etc/bind/named.conf

cat /etc/bind/null.zone.template | \
    sed s,'/%TIMESTAMP%/',"${TIMESTAMP}",g | \
    sed s,'/%FQDN%/',"${FQDN}",g | \
    sed s,'/%IPV4%/',"${IPV4}",g | \
    sed s,'/%IPV6%/',"${IPV6}",g > \
    /etc/bind/null.zone

mkdir -p /logs
touch /logs/rpz_log.txt
chmod a+rwx -R /logs

named -f -u named
