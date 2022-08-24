#!/bin/sh

export TLS_ENABLED="${TLS_ENABLED:-false}"
export TLS_EMAIL="${TLS_EMAIL:-user@example.com}"
export TLS_DOMAIN="${TLS_DOMAIN:-dns.example.com}"
export BACKEND_PORT="${BACKEND_PORT:-1153}"
export PORT="${PORT:-53}"

run_certbot() {
	certbot certonly --standalone \
		--non-interactive --agree-tos \
		--preferred-chain="ISRG Root X1" \
		-d ${TLS_DOMAIN} -m ${TLS_EMAIL}

	cp /etc/letsencrypt/live/${TLS_DOMAIN}/*.pem /data/certs/.
	chmod -R a+r /etc/letsencrypt/archive
	chmod -R a+r /data/certs/
}

run_certbot_init() {
	echo "[certbot] registering ssl cert"
	run_certbot
	echo "[certbot] registering ssl cert complete"
}

run_certbot_update() {
	while true; do
		sleep 3600

		echo "[certbot] updating ssl cert"
		run_certbot
		echo "[certbot] updating ssl cert complete"

		echo "[certbot] reloading ssl cert"
		dnsdist -c 127.0.0.1 -k miQjUydO7fwUmSDS0hT+2pHC1VqT8vOjfexOyvHKcNA= -e "reloadAllCertificates()"
		echo "[certbot] reloading ssl cert complete"
	done
}

run_dnstap() {
	echo "[logs] running dnstap"
	mkdir -p /var/run/dnstap/
	/usr/bin/dnstap -y \
		-u /var/run/dnstap/dnstap.sock \
		-a -w /logs/logs.yaml
}

run_empty_log_file() {
	while true; do
		echo "[logs] emptying log file"
		echo "" >/logs/logs.yaml
		echo "[logs] emptying log file complete"

		sleep 600
	done
}

run_dnsdist() {
	echo "[dnsdist] running dnsdist"
	dnsdist --uid dnsdist --gid dnsdist --supervised --disable-syslog
}

PID_LIST=""

# Runs certbot
if [ $TLS_ENABLED == "true" ]; then
	echo "[certbot] TLS_ENABLED=true - running certbot"
	# Runs certbot first time
	run_certbot_init

	# Runs certbot update
	run_certbot_update &
	PID_LIST="$PID_LIST $!"
else
	# Skips running certbot
	echo "[certbot] TLS_ENABLED=false - skip running certbot"
fi

# Runs dnstap
run_dnstap &
PID_LIST="$PID_LIST $!"
run_empty_log_file &
PID_LIST="$PID_LIST $!"

# Runs dnsdist
run_dnsdist &
PID_LIST="$PID_LIST $!"

trap "kill $PID_LIST" SIGINT SIGKILL EXIT
wait $PID_LIST
exit 0
