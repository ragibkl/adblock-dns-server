#!/usr/bin/env bash

TAG=${1:-default}
docker push ragibkl/adblock_dns:$TAG
