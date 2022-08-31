#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
WORKDIR="$SCRIPT_DIR"/..

BRANCH=$(git describe --tags --exact-match 2> /dev/null \
  || git symbolic-ref -q --short HEAD \
  || git rev-parse --short HEAD)

TAG=latest
if [ "$BRANCH" != master ]
then
    TAG=$BRANCH
fi

REGISTRY_TAG="ragibkl/adblock_dnsdist:$TAG"
echo "WORKDIR=$WORKDIR"
echo "BRANCH=$BRANCH"
echo "TAG=$TAG"
echo "REGISTRY_TAG=$REGISTRY_TAG"

cd $WORKDIR
docker build --pull -t $REGISTRY_TAG -f ./Dockerfile .
docker push $REGISTRY_TAG
