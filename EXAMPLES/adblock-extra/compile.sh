#!/bin/bash

touch data/output.d/blacklist.zone

docker pull ragibkl/adblock_compiler:rust
docker run -ti --rm \
    -v $PWD/../../data/:/data/ \
    -v $PWD/data/blacklist.d/extra_blacklist.hosts:/data/blacklist.d/__extra_blacklist.hosts \
    -v $PWD/data/whitelist.d/extra_whitelist.hosts:/data/whitelist.d/__extra_whitelist.hosts \
    -v $PWD/data/output.d/blacklist.zone:/data/output.d/blacklist.zone \
    ragibkl/adblock_compiler:rust
