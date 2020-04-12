#!/bin/bash

cd dns_default

# compile blacklist
touch output.d/blacklist.zone
docker pull ragibkl/adblock_compiler:rust
docker run \
    --rm \
    -e TERM="xterm" \
    -v $PWD/../data/:/data/ \
    -v $PWD/output.d/blacklist.zone:/data/output.d/blacklist.zone \
    ragibkl/adblock_compiler:rust

# build image
docker build --pull -t ragibkl/adblock_dns:ci-build .
# docker tag ragibkl/adblock_dns:default ragibkl/adblock_dns:latest

# push image
docker push ragibkl/adblock_dns:ci-build
# docker push ragibkl/adblock_dns:latest
