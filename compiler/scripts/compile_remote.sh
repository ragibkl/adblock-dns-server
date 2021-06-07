#!/usr/bin/bash

cargo run -- \
    -f https://raw.githubusercontent.com/ragibkl/adblock-dns-server/master/data/configuration.yaml\
    -o blacklist.zone
