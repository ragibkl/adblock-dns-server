#!/usr/bin/env bash

# compile blacklist
touch output.d/blacklist.zone
docker pull ragibkl/adblock_compiler:rust
docker run \
    -ti --rm --init \
    -v $PWD/../data/:/data/ \
    -v $PWD/output.d/blacklist.zone:/data/output.d/blacklist.zone \
    ragibkl/adblock_compiler:rust

# build image
docker build --pull -t ragibkl/adblock_dns:default .

# push image
docker push ragibkl/adblock_dns:default

docker tag ragibkl/adblock_dns:default ragibkl/adblock_dns:latest
docker push ragibkl/adblock_dns:latest
