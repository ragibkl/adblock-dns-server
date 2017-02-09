#!/bin/sh

python3 main.py;
docker-compose --build
docker-compose up -d