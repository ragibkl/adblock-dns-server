#!/usr/bin/env bash

docker build --pull -t ragibkl/adblock_logs_viewer .
docker push ragibkl/adblock_logs_viewer
