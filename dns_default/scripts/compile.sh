#!/usr/bin/env bash

touch output.d/blacklist.zone

# docker pull ragibkl/adblock_compiler
docker run -ti --rm \
    -v $PWD/../data/:/data/ \
    -v $PWD/output.d/blacklist.zone:/data/output.d/blacklist.zone \
    ragibkl/adblock_compiler
