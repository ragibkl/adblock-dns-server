#!/usr/bin/env bash

TAG=${1:-latest}
touch output.d/blacklist.zone

docker pull ragibkl/adblock_compiler:$TAG
docker run -ti --rm \
    -v $PWD/output.d/blacklist.zone:/data/output.d/blacklist.zone \
    ragibkl/adblock_compiler:$TAG
