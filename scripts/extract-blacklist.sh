#!/bin/bash

docker pull ragibkl/adblock_dns
docker run --rm -ti \
    -v $(pwd)/temp:/temp_data \
    --entrypoint=cp \
    ragibkl/adblock_dns \
    /etc/bind/blacklist.zone /temp_data/.
