#!/bin/bash

WORKDIR=$(pwd)
BRANCH=$(git describe --tags --exact-match 2> /dev/null \
  || git symbolic-ref -q --short HEAD \
  || git rev-parse --short HEAD)

TAG=latest
if [ "$BRANCH" != master ]
then
    TAG=$BRANCH
fi

echo "TAG=$TAG"

cd $WORKDIR/adblock_http
./scripts/build.sh $TAG
./scripts/push.sh $TAG

cd $WORKDIR/compiler
./scripts/build.sh $TAG
./scripts/push.sh $TAG

cd $WORKDIR/dns
./scripts/compile.sh $TAG
./scripts/build.sh $TAG
./scripts/push.sh $TAG
