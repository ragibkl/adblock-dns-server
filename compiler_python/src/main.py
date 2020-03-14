import os

import http_loader
import file_loader
import extractor
from blacklist_writer import write_to_file

from settings import (
    HTTP_BLACKLIST_PATH,
    CUSTOM_BLACKLIST_DIR,
    CUSTOM_WHITELIST_DIR,
    CUSTOM_OVERRIDES_DIR,
    BLACKLIST_OUTPUT_PATH,
)


def main():
    urls = http_loader.get_urls_from_file(HTTP_BLACKLIST_PATH)
    domains = http_loader.load_domains_for_urls(urls)

    paths = file_loader.get_paths_in_dir(CUSTOM_BLACKLIST_DIR)
    for path in paths:
        path_domains = file_loader.load_domains_for_file(path)
        domains.extend(list(path_domains))

    whitelist_domains = []
    paths = file_loader.get_paths_in_dir(CUSTOM_WHITELIST_DIR)
    for path in paths:
        path_domains = file_loader.load_domains_for_file(path)
        whitelist_domains.extend(list(path_domains))

    overrides = []
    override_paths = file_loader.get_paths_in_dir(CUSTOM_OVERRIDES_DIR)
    for path in override_paths:
        path_overrides = file_loader.load_overrides_for_file(path)
        overrides.extend(list(path_overrides))

        override_domains = file_loader.load_domains_for_file(path)
        whitelist_domains.extend(override_domains)

    domains = extractor.dedup_domains(domains)
    domains = extractor.exclude_whitelist_domains(domains, whitelist_domains)
    domains = extractor.sort_domains(domains)

    write_to_file(domains, overrides, BLACKLIST_OUTPUT_PATH)


if __name__ == "__main__":
    main()
