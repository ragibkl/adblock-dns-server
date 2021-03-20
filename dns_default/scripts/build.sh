#!/usr/bin/env bash

# build image
docker build --pull --no-cache \
    -t ragibkl/adblock_dns:default \
    -f ./Dockerfile ..
docker tag ragibkl/adblock_dns:default ragibkl/adblock_dns:latest

# push image
docker push ragibkl/adblock_dns:default
docker push ragibkl/adblock_dns:latest
