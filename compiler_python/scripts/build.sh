#!/usr/bin/env bash

TAG=${1:-latest}

pipenv lock -r > requirements.txt
docker build --pull -t ragibkl/adblock_compiler:$TAG .
