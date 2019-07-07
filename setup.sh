#!/bin/sh

WORKDIR=$(pwd)
FILES_DIR=${WORKDIR}/files

docker run \
    -v ${FILES_DIR}:/app/files \
    --env-file=.env \
    ragibkl/adblock_compiler
