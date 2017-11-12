from datetime import datetime

class NullzoneWriter:
    """docstring for ConfigBuilder."""
    def __init__(self, output_path, ipv4='127.0.0.1', ipv6=None, domain='example.org', host='dns'):
        self.output_path = output_path
        self.ipv4 = ipv4
        self.ipv6 = ipv6
        self.domain = domain
        self.host = host

    def export_to_file(self):
        content = self.get_config_content()
        self.write_to_file(content)

    def get_config_content(self):
        null_zone_lines = [
            '$TTL   86400       ; one day',
            '@      IN      SOA     {}.     hostmaster.{}. ('.format(self.domain, self.domain),
            '       {}  ; serial number YYMMDDHH'.format(datetime.now().strftime("%Y%m%d%H")),
            '       28800       ; refresh   8 hours',
            '       7200        ; retry     2 hours',
            '       864000      ; expire    10 days',
            '       86400)      ; min ttl   1 day',
            '               NS      {}.{}.'.format(self.host, self.domain),
            '               A       {}'.format(self.ipv4),
            '*      IN      A       {}'.format(self.ipv4),
            '       IN      AAAA    {}'.format(self.ipv6) if self.ipv6 else None,
            '',
        ]
        null_zone_lines = [line for line in null_zone_lines if line is not None]

        return '\n'.join(null_zone_lines)

    def write_to_file(self, content):
        file_name = self.output_path

        output_file = open(file_name, 'w')
        output_file.write(content)
        output_file.close()

        print('Created "nullzone" file: "{}"'.format(file_name))
