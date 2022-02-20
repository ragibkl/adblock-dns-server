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

run_certbot () {
    echo "registering ssl cert";
    certbot certonly --standalone \
        --non-interactive --agree-tos \
        -d ${DOH_DOMAIN} -m ${DOH_EMAIL};
    echo "registering ssl cert completed";

    while true
    do
        sleep 3600;

        echo "updating ssl cert";
        certbot certonly --standalone \
            --non-interactive --agree-tos \
            -d ${DOH_DOMAIN} -m ${DOH_EMAIL};
        echo "updating ssl cert complete";
    done
}

run_dnsdist () {
    echo "running dnsdist";
    dnsdist --uid dnsdist --gid dnsdist --supervised --disable-syslog
}

PID_LIST=""
# Runs certbot
run_certbot & PID_LIST="$PID_LIST $!";

# Runs dnsdist
sleep 5;
run_dnsdist & PID_LIST="$PID_LIST $!";

trap "kill $PID_LIST" SIGINT;
wait $PID_LIST;
exit 0;
