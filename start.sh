#!/bin/sh

cp -n sample.env .env
docker-compose pull
docker-compose up -d
