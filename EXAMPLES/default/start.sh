#!/bin/sh

COMPOSE="docker compose"
docker compose version >/dev/null 2>&1 || COMPOSE="docker-compose"

cp -n sample.env .env
$COMPOSE pull
$COMPOSE up -d --remove-orphans
