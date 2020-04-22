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

echo "WORKDIR=$WORKDIR"
echo "BRANCH=$BRANCH"
echo "TAG=$TAG"

# build the adblock_compiler:rust image
# cd $WORKDIR/compiler
# ./scripts/build.sh

# build the adblock_dns:default
cd $WORKDIR/dns_default
./scripts/ci-build.sh
