#!/bin/sh

TIMESTAMP=$(date +"%Y%m%d%H")
FQDN="${FQDN:-localhost.localdomain}"
IPV4="${IPV4:-127.0.0.1}"
IPV6="${IPV6:-::1}"

cat /etc/bind/null.zone.template |
	sed s,'/%TIMESTAMP%/',"${TIMESTAMP}",g |
	sed s,'/%FQDN%/',"${FQDN}",g |
	sed s,'/%IPV4%/',"${IPV4}",g |
	sed s,'/%IPV6%/',"${IPV6}",g >/etc/bind/null.zone

mkdir -p /logs
touch /logs/rpz_log.txt
chmod a+rwx -R /logs

named -f -u named
