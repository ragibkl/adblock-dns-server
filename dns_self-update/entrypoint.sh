#!/bin/sh

TIMESTAMP=$(date +"%Y%m%d%H")
FQDN="${FQDN:-localhost.localdomain}"
IPV4="${IPV4:-127.0.0.1}"
IPV6="${IPV6:-::1}"
CONFIG_URL="${CONFIG_URL:-https://raw.githubusercontent.com/ragibkl/adblock-dns-server/master/data/configuration.yaml}"

cat /etc/bind/null.zone.template | \
    sed s,'/%TIMESTAMP%/',"${TIMESTAMP}",g | \
    sed s,'/%FQDN%/',"${FQDN}",g | \
    sed s,'/%IPV4%/',"${IPV4}",g | \
    sed s,'/%IPV6%/',"${IPV6}",g > \
    /etc/bind/null.zone

mkdir -p /logs
touch /logs/rpz_log.txt 
chmod a+rwx -R /logs

run_bind () {
    named -f -u named;
}

update () {
    while true
    do
        echo "updating blacklist";
        compiler \
            -f $CONFIG_URL \
            -o /etc/bind/blacklist.zone;
        echo "updating blacklist complete";

        rndc reload;
        sleep 3600;
    done
}

PID_LIST=""
run_bind & PID_LIST="$PID_LIST $!";
update & PID_LIST="$PID_LIST $!";

trap "kill $PID_LIST" SIGINT;
wait $PID_LIST;
exit 0;
