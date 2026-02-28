#!/bin/sh

SERVERS="
    sg-dns1.bancuh.com
    sg-dns2.bancuh.com
    fr-dns1.bancuh.com
    fr-dns2.bancuh.com
    jp-dns1.bancuh.com
    jp-dns2.bancuh.com
    us-dns1.bancuh.com
"

for server in $SERVERS; do
    echo "==> $server"
    ssh root@$server "cd /root/adblock-dns-server && git pull && cd EXAMPLES/default && ./start.sh"
    echo ""
done
