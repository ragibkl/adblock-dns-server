#!/usr/bin/env bash

pipenv lock -r > requirements.txt

docker build -t ragibkl/adblock_compiler:v2 .

rm requirements.txt
