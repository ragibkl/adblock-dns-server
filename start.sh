#!/bin/sh

cp sample.env .env -n
docker-compose pull
docker-compose up -d
