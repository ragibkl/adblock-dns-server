#!/bin/sh

TLS_ENABLED="${TLS_ENABLED:-false}"
TLS_EMAIL="${TLS_EMAIL:-user@example.com}"
TLS_DOMAIN="${TLS_DOMAIN:-dns.example.com}"

run_certbot() {
    certbot certonly --standalone \
        --non-interactive --agree-tos \
        --preferred-chain="ISRG Root X1" \
        -d ${TLS_DOMAIN} -m ${TLS_EMAIL};
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

run_dnstap () {
    echo "running dnstap";
    mkdir -p /var/run/dnstap/;
    /usr/bin/dnstap -y \
        -u /var/run/dnstap/dnstap.sock \
        -a -w /logs/logs.yaml;
}

run_dnsdist () {
    echo "running dnsdist";
    dnsdist --uid dnsdist --gid dnsdist --supervised --disable-syslog;
}

PID_LIST=""

# Runs certbot
if [ $TLS_ENABLED == "true" ]
then
    # Runs certbot first time
    run_certbot_init

    # Runs certbot update
    run_certbot_update & PID_LIST="$PID_LIST $!";
else
    # Skips running certbot
    echo "skip running certbot"
fi


# Runs dnstap
run_dnstap & PID_LIST="$PID_LIST $!";

# Runs dnsdist
run_dnsdist & PID_LIST="$PID_LIST $!";

trap "kill $PID_LIST" SIGINT;
wait $PID_LIST;
exit 0;
