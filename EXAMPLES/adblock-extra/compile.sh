#!/bin/bash

touch data/output.d/blacklist.zone

docker pull ragibkl/adblock_compiler:rust
docker run -ti --rm \
    -ti --rm --init \
    -v $PWD/../../data/:/data/ \
    -v $PWD/data/output.d/blacklist.zone:/data/output.d/blacklist.zone \
    ragibkl/adblock_compiler:rust
