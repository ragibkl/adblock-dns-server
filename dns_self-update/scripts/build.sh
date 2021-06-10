#!/usr/bin/env bash

# build image
docker build --pull --no-cache \
    -t ragibkl/adblock_dns:self-update \
    -f ./Dockerfile .

# push image
docker push ragibkl/adblock_dns:self-update
