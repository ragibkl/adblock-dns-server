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

# build the adblock_dns image
# ./adblock_dns/scripts/build.sh

# build the adblock_dnsdist:latest image
# ./adblock_dnsdist/scripts/build.sh

# build the adblock_dnsdist_logs:latest image
# ./adblock_dnsdist_logs/scripts/build.sh
