#!/usr/bin/env bash

./replace.sh

cp config/named.conf.local /etc/bind/named.conf.local
cp config/named.conf.options /etc/bind/named.conf.options
cp config/null.zone.file /etc/bind/null.zone.file
cp config/blacklist.zone /etc/bind/blacklist.zone

named -g
