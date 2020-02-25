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

cd $WORKDIR/compiler_rust
cargo run --release

cd $WORKDIR/dns
cp $WORKDIR/compiler_rust/data/output.d/blacklist.zone $WORKDIR/dns/output.d/blacklist.zone
./scripts/build.sh $TAG
./scripts/push.sh $TAG
