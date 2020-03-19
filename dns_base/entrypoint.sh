#!/bin/sh

./replace.sh

cp config/named.conf.local /etc/bind/named.conf.local
cp config/named.conf.options /etc/bind/named.conf.options
cp config/null.zone.file /etc/bind/null.zone.file

named -g -u named
