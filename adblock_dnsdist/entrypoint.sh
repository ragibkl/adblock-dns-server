#!/bin/sh

DOH_ENABLED="${DOH_ENABLED:-false}"
DOH_EMAIL="${DOH_EMAIL:-user@example.com}"
DOH_DOMAIN="${DOH_DOMAIN:-dns.example.com}"
IPV4="${IPV4:-127.0.0.1}"
IPV6="${IPV6:-::1}"

cat /etc/dnsdist.template.conf | \
    sed s,'/%DOH_DOMAIN%/',"${DOH_DOMAIN}",g | \
    sed s,'/%IPV4%/',"${IPV4}",g | \
    sed s,'/%IPV6%/',"${IPV6}",g > \
    /etc/dnsdist.conf

run_certbot() {
    certbot certonly --standalone \
        --non-interactive --agree-tos \
        --preferred-chain="ISRG Root X1" \
        -d ${DOH_DOMAIN} -m ${DOH_EMAIL};
}

run_certbot_init() {
    echo "registering ssl cert";
    run_certbot;
    echo "registering ssl cert complete";
}

run_certbot_update () {
    while true
    do
        sleep 3600;

        echo "updating ssl cert";
        run_certbot;
        echo "updating ssl cert complete";

        echo "reloading ssl cert";
        dnsdist -c 127.0.0.1 -k miQjUydO7fwUmSDS0hT+2pHC1VqT8vOjfexOyvHKcNA= -e "reloadAllCertificates()"
        echo "reloading ssl cert complete";
    done
}

run_dnsdist () {
    echo "running dnsdist";
    dnsdist --uid dnsdist --gid dnsdist --supervised --disable-syslog;
}

# Runs certbot first time
run_certbot_init

PID_LIST=""

# Runs dnsdist
run_dnsdist & PID_LIST="$PID_LIST $!";

# Runs certbot update
run_certbot_update & PID_LIST="$PID_LIST $!";

trap "kill $PID_LIST" SIGINT;
wait $PID_LIST;
exit 0;
