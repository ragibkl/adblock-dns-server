import os
import socket
import sys, shutil


BASE_DIR = os.path.dirname(os.path.abspath(__file__)),
# BADLIST_PATH = '/etc/bind/badlist'

BADLIST_PATH = os.path.join(BASE_DIR, os.path.abspath('files/badlist'))
NULL_DOMAIN = 'dns3.bancuh.com.'
NULL_ZONE_FILE_PATH = os.path.join(BASE_DIR, os.path.abspath('files/null.zone.file'))
NAMED_CONFIG_PATH = os.path.join(BASE_DIR, os.path.abspath('files/named/'))
NAMED_MAIN_CONFIG = os.path.join(BASE_DIR, os.path.abspath('files/named.conf'))

ADBLOCK_SOURCES = [
    'http://adaway.org/hosts.txt',
    'http://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&mimetype=plaintext',
    'http://winhelp2002.mvps.org/hosts.txt',
    'http://hosts-file.net/ad_servers.txt',
]

FQDN = socket.getfqdn()
IP4 = socket.gethostbyname(FQDN)
IP6 = None

