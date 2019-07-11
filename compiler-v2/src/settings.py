import os

DEFAULT_DATA_DIR = os.path.join(os.getcwd(), 'data')
DATA_DIR = os.getenv('DATA_DIR', default=DEFAULT_DATA_DIR)

# Local custom blacklist files
CUSTOM_BLACKLIST_DIR = os.getenv(
    'CUSTOM_BLACKLIST_DIR',
    default=os.path.join(DATA_DIR, os.path.abspath('blacklist.d'))
)

# Http sources for blacklist sources
HTTP_BLACKLIST_PATH = os.getenv(
    'HTTP_BLACKLIST_PATH',
    default=os.path.join(DATA_DIR, os.path.abspath('blacklist-urls.txt'))
)

# Output blacklist path
BLACKLIST_OUTPUT_PATH = os.getenv(
    'BLACKLIST_OUTPUT_PATH',
    default=os.path.join(DATA_DIR, os.path.abspath('blacklist.zone'))
)

# TODO: maybe we don't need these anymore
# HOST = os.getenv('HOST', default='dns1')
# DOMAIN = os.getenv('DOMAIN', default='localhost.localdomain')
# IPV4 = os.getenv('IPV4', '127.0.0.1')
# IPV6 = os.getenv('IPV6', None)
