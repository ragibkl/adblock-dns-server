#!/usr/bin/env bash

docker build --pull -t ragibkl/adblock_dnsdist -f ./Dockerfile .
docker push ragibkl/adblock_dnsdist
