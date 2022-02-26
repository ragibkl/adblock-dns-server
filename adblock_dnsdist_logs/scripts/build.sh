#!/usr/bin/env bash

docker build --pull -t ragibkl/adblock_dnsdist_logs .
docker push ragibkl/adblock_dnsdist_logs
