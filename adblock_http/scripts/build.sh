#!/usr/bin/env bash

TAG=${1:-latest}
docker build --pull -t ragibkl/adblock_http:$TAG .
