import os

import http_loader
import file_loader
import extractor
from blacklist_writer import write_to_file

from settings import (
    HTTP_BLACKLIST_PATH,
    CUSTOM_BLACKLIST_DIR,
    CUSTOM_WHITELIST_DIR,
    BLACKLIST_OUTPUT_PATH,
)


def main():
    domains = []

    urls = http_loader.get_urls_from_file(HTTP_BLACKLIST_PATH)
    for url in urls:
        url_domains = http_loader.load_domains_for_url(url)
        domains.extend(list(url_domains))

    paths = file_loader.get_paths_in_dir(CUSTOM_BLACKLIST_DIR)
    for path in paths:
        path_domains = file_loader.load_domains_for_file(path)
        domains.extend(list(path_domains))

    whitelist_domains = []
    paths = file_loader.get_paths_in_dir(CUSTOM_WHITELIST_DIR)
    for path in paths:
        path_domains = file_loader.load_domains_for_file(path)
        whitelist_domains.extend(list(path_domains))

    domains = extractor.dedup_domains(domains)
    domains = extractor.exclude_whitelist_domains(domains, whitelist_domains)
    write_to_file(domains, BLACKLIST_OUTPUT_PATH)


if __name__ == "__main__":
    main()
