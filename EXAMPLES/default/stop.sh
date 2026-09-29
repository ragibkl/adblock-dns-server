#!/bin/sh

if ! docker compose version >/dev/null 2>&1; then
    echo "This needs Docker with the Compose v2 plugin ('docker compose')." >&2
    echo "See https://docs.docker.com/compose/install/" >&2
    exit 1
fi

docker compose down
