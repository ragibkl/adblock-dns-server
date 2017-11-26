import os
import socket
import sys, shutil


BASE_DIR = os.path.dirname(os.path.abspath(__file__))

BLACKLIST_FILEPATH = os.path.join(BASE_DIR, os.path.abspath('files/badlist'))
NULLZONE_FILEPATH = os.path.join(BASE_DIR, os.path.abspath('files/null.zone.file'))
NAMED_CONFIG_PATH = os.path.join(BASE_DIR, os.path.abspath('files/named/'))
NAMED_MAIN_CONFIG = os.path.join(BASE_DIR, os.path.abspath('files/named.conf'))

ADBLOCK_SOURCES = [
    'http://adaway.org/hosts.txt',
    'http://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext',
    'http://winhelp2002.mvps.org/hosts.txt',
    'http://hosts-file.net/ad_servers.txt',
    'https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts',
    'local_sources/porn_custom.txt',
    'local_sources/porn_reddit.txt',
]

HOST = 'dns1'
DOMAIN = 'example.org'
IPV4 = '0.0.0.0'
IPV6 = None
