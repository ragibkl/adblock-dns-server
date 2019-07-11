#!/usr/bin/env bash

touch templates/blacklist.zone
docker run -ti \
    -v $PWD/templates/blacklist.zone:/data/output.d/blacklist.zone \
    ragibkl/adblock_compiler:v2
