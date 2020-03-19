#!/usr/bin/env bash

TAG=${1:-default}

# compile blacklist
touch output.d/blacklist.zone
docker pull ragibkl/adblock_compiler:rust
docker run \
    -ti --rm --init \
    -v $PWD/../data/:/data/ \
    -v $PWD/output.d/blacklist.zone:/data/output.d/blacklist.zone \
    ragibkl/adblock_compiler:rust

# build image
docker build --pull -t ragibkl/adblock_dns:$TAG .

# push image
docker push ragibkl/adblock_dns:$TAG
