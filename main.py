#!python3
from datetime import datetime

import settings
from domain_utils.crawler import Crawler
from domain_utils.blacklist_writer import BlacklistWriter

def create_null_zone():
    null_zone_file_path = settings.NULL_ZONE_FILE_PATH
    fqdn = settings.FQDN
    ip6 = settings.IP6
    ip4 = settings.IP4
    domain_parts = fqdn.split('.')

    # set blacklist server ip
    host = domain_parts[0]
    del domain_parts[0]
    if len(domain_parts) == 0:
        domain = 'example.com'
    else:
        domain = '.'.join(domain_parts)

    null_zone_lines = [
        '$TTL   86400       ; one day',
        '@      IN      SOA     %s.     hostmaster.%s. (' % (domain, domain),
        '       %s  ; serial number YYMMDDHH' % (datetime.now().strftime("%Y%m%d%H")),
        '       28800       ; refresh   8 hours',
        '       7200        ; retry     2 hours',
        '       864000      ; expire    10 days',
        '       86400)      ; min ttl   1 day',
        '               NS      %s.%s.' % (host, domain),
        '               A       %s' % (ip4),
        '*      IN      A       %s' % (ip4)
    ]
    if ip6 and ip6 != '':
        null_zone_lines.append('       IN	AAAA	%s' % (ip6))

    null_zone_content = '\n'.join(null_zone_lines)
    print(null_zone_content)
    null_zone_file = open(null_zone_file_path, 'w')
    null_zone_file.write(null_zone_content)
    null_zone_file.close()


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
