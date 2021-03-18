#!/usr/bin/env bash

TAG=${1:-rust}

docker build --pull -t ragibkl/adblock_compiler:$TAG .
docker push ragibkl/adblock_compiler:$TAG
