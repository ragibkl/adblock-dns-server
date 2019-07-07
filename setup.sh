#!/bin/sh

WORKDIR=$(pwd)
FILES_DIR=${WORKDIR}/files

docker run -v ${FILES_DIR}:/app/files ragibkl/adblock_compiler
