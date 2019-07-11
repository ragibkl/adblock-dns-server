import os

import http_loader
import file_loader
import extractor

from settings import HTTP_BLACKLIST_PATH, CUSTOM_BLACKLIST_DIR


def main():
    domains = []

    urls = http_loader.get_urls_from_file(HTTP_BLACKLIST_PATH)
    for url in urls:
        url_domains = http_loader.load_domains_for_url(url)
        domains.extend(list(url_domains))

    paths = os.listdir(CUSTOM_BLACKLIST_DIR)
    for path in paths:
        path_domains = file_loader.load_domains_for_file(path)
        domains.extend(list(path_domains))

    domains = list(set(domains))


if __name__ == "__main__":
    main()
