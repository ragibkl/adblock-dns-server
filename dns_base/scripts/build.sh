#!/usr/bin/env bash

docker build --pull -t ragibkl/adblock_dns:base -f ./Dockerfile ..
docker push ragibkl/adblock_dns:base
