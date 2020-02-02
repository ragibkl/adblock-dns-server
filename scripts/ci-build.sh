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

echo $BRANCH
echo $TAG

cd compiler
./scripts/build.sh $TAG
./scripts/push.sh $TAG
cd $WORKDIR

cd dns
./scripts/compile.sh $TAG
./scripts/build.sh $TAG
./scripts/push.sh $TAG
cd $WORKDIR
