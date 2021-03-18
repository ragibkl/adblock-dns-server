#!/usr/bin/env bash

# copy data files
cp -rf ../data/ data

# build image
docker build --pull --no-cache -t ragibkl/adblock_dns:default .
docker tag ragibkl/adblock_dns:default ragibkl/adblock_dns:latest

# push image
docker push ragibkl/adblock_dns:default
docker push ragibkl/adblock_dns:latest
