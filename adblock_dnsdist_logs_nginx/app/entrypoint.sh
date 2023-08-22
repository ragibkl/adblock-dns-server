#!/usr/bin/env bash

./replace.sh
cp nginx.conf /etc/nginx/conf.d/default.conf
nginx -g 'daemon off;'
