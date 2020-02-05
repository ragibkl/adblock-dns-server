#!/usr/bin/env bash

TAG=${1:-latest}
docker push ragibkl/adblock_http:$TAG
