#!/usr/bin/env bash

TAG=${1:-rust}

cp -r ../data/. data/.

docker build -t ragibkl/adblock_compiler:$TAG .
docker push ragibkl/adblock_compiler:$TAG
