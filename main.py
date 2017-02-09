#!python3
import requests
from datetime import datetime

import settings


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
    null_domain = settings.NULL_DOMAIN
    null_ip = settings.IP4
    output_path = settings.BADLIST_PATH

    sources = settings.ADBLOCK_SOURCES

    ad_domain_list = []

    for source in sources :
        print('Started downloading from : %s' % (source))
        response = requests.get(source)
        openfile = str(response.content, 'UTF-8')
        lines = openfile.replace('\r','').split('\n')
        for line in lines :
            a = line.replace('\t',' ').split(' ')
            if a[0] == '#':
                print('Rejected : %s' % (line))

            elif len(a) < 2:
                print('Rejected : %s' % (line))

            elif '_' in a[1]:
                print('Rejected : %s' % (line))

            elif len(a[1]) > 1 and a[1][-1] == '.' :
                print ('Rejected : %s' % (line))

            elif a[1] in ('','localhost'):
                print('Rejected : %s' % (line))

            elif a[0] in ['0.0.0.0', '127.0.0.1'] and a[1][-1] != '.':
                ad_domain_list.append(a[1].lower())
        print('Finished downloading from : %s' % (source) )

    print('Before del duplicates, row count = %d' % (len(ad_domain_list)))
    ad_domain_list = list(set(ad_domain_list))
    print('After de-duplicate, row count = %d' % (len(ad_domain_list)))

    adblock_zone_list = [
        '$TTL 1H',
        '@               SOA     LOCALHOST. named-mgr.example.com (1 1h 15m 30d 2h)',
        '                NS      LOCALHOST.',
        '',
    ]
    for ad_domain in ad_domain_list :
        # adblock_zone_list.append('{line}\tCNAME\t{null_domain}'.format(line=line, null_domain=null_domain) )
        adblock_zone_list.append('{ad_domain}\t\t\tA\t{null_domain}'.format(ad_domain=ad_domain, null_domain=null_ip))

    adblock_zone_list.append('')
    string = '\n'.join(adblock_zone_list)
    print("Created file %s with %d zones." % (output_path,len(adblock_zone_list)))

    output_file = open(output_path,'w')
    output_file.write(string)
    output_file.close()


if __name__ == '__main__':
    create_null_zone()
    update_badlist()
