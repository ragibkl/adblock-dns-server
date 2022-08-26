#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
WORK_DIR="$SCRIPT_DIR"/..
cd $WORK_DIR

docker build --pull -t ragibkl/adblock_dns:latest .

# tag others
# docker tag ragibkl/adblock_dns:latest ragibkl/adblock_dns:default
# docker tag ragibkl/adblock_dns:latest ragibkl/adblock_dns:self-update

# push tags
# docker push ragibkl/adblock_dns:self-update
# docker push ragibkl/adblock_dns:default
docker push ragibkl/adblock_dns:latest
