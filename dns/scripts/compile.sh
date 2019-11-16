#!/usr/bin/env bash

touch templates/blacklist.zone

docker pull ragibkl/adblock_compiler
docker run -ti --rm \
    -v $PWD/templates/blacklist.zone:/data/output.d/blacklist.zone \
    ragibkl/adblock_compiler
