#!/usr/bin/env bash

docker build --pull -t ragibkl/adblock_dns:base .
docker push ragibkl/adblock_dns:base
