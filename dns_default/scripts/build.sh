#!/usr/bin/env bash

TAG=${1:-default}
docker build --pull -t ragibkl/adblock_dns:$TAG .
