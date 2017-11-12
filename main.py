#!python3
from datetime import datetime

import settings
from domain_utils.crawler import Crawler
from domain_utils.blacklist_writer import BlacklistWriter
from domain_utils.nullzone_writer import NullzoneWriter


def create_null_zone():
    null_zone_file_path = settings.NULL_ZONE_FILE_PATH
    fqdn = settings.FQDN
    ipv6 = settings.IP6
    ipv4 = settings.IP4
    domain_parts = fqdn.split('.')

    # set blacklist server ip
    host = domain_parts[0]
    del domain_parts[0]
    if len(domain_parts) == 0:
        domain = 'example.com'
    else:
        domain = '.'.join(domain_parts)

    nullzone_writer = NullzoneWriter(null_zone_file_path, ipv4, ipv6, domain, host)
    nullzone_writer.export_to_file()


def update_badlist():
    redirect_ip = settings.IP4
    output_path = settings.BADLIST_PATH
    sources = settings.ADBLOCK_SOURCES

    ad_domain_list = []
    for source in sources :
        crawler = Crawler(source)
        ad_domain_list.extend(crawler.get_domains())
    ad_domain_list = Crawler.remove_duplicates(ad_domain_list)

    blacklist_writer = BlacklistWriter(ad_domain_list, redirect_ip, output_path)
    blacklist_writer.export_to_file()


if __name__ == '__main__':
    create_null_zone()
    update_badlist()
