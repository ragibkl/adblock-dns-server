#!/bin/sh

export TIMESTAMP=$(date +"%Y%m%d%H")
export FQDN="${FQDN:-localhost.localdomain}"
export IPV4="${IPV4:-127.0.0.1}"
export IPV6="${IPV6:-::1}"
export CONFIG_URL="${CONFIG_URL:-https://raw.githubusercontent.com/ragibkl/adblock-dns-server/master/data/configuration.yaml}"

ENV_VARS=$(printf '${%s} ' $(env | sed 's/=.*//'))
envsubst "$ENV_VARS" < /etc/bind/null.zone.template > /etc/bind/null.zone

mkdir -p /logs
touch /logs/rpz_log.txt
chmod a+rw -R /logs

run_bind() {
	named -f -u named
}

run_update() {
	while true; do
		echo "updating blacklist"
		compiler \
			-f $CONFIG_URL \
			-o /etc/bind/blacklist.zone
		echo "updating blacklist complete"

		rndc reload
		sleep 3600
	done
}

PID_LIST=""

run_bind &
PID_LIST="$PID_LIST $!"

run_update &
PID_LIST="$PID_LIST $!"

trap "kill $PID_LIST" SIGINT
wait $PID_LIST
exit 0
