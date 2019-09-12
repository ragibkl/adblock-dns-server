#!/usr/bin/env bash

pipenv lock -r > requirements.txt
docker build --pull -t ragibkl/adblock_compiler:latest .
