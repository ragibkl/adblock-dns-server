import os

DEFAULT_DATA_DIR = os.path.join(os.getcwd(), 'data')
DATA_DIR = os.getenv('DATA_DIR', default=DEFAULT_DATA_DIR)

# Local custom blacklist files
CUSTOM_BLACKLIST_DIR = os.getenv(
    'CUSTOM_BLACKLIST_DIR',
    default=os.path.join(DATA_DIR, 'blacklist.d')
)

# Http sources for blacklist sources
HTTP_BLACKLIST_PATH = os.getenv(
    'HTTP_BLACKLIST_PATH',
    default=os.path.join(DATA_DIR, 'blacklist-src-urls.txt')
)

# Local custom whitelist files
CUSTOM_WHITELIST_DIR = os.getenv(
    'CUSTOM_BLACKLIST_DIR',
    default=os.path.join(DATA_DIR, 'whitelist.d')
)

# Custom domain overrides files
CUSTOM_OVERRIDES_DIR = os.getenv(
    'CUSTOM_OVERRIDES_DIR',
    default=os.path.join(DATA_DIR, 'overrides.d')
)

# Output blacklist path
BLACKLIST_OUTPUT_PATH = os.getenv(
    'BLACKLIST_OUTPUT_PATH',
    default=os.path.join(DATA_DIR, 'output.d/blacklist.zone')
)
