class BlacklistWriter:
    """Helper class to help export the blacklist to the output file
    also formats the blacklist in the correct format.
    """

    headers = [
        '$TTL 1H',
        '@               SOA     LOCALHOST. named-mgr.example.com (1 1h 15m 30d 2h)',
        '                NS      LOCALHOST.',
    ]

    _padding_size = 0

    @property
    def padding_size(self):
        if not self._padding_size:
            maxlength = max(len(s) for s in self.domain_list)
            self._padding_size = maxlength

        return self._padding_size

    def __init__(self, domain_list, overrides, output_path):
        self.domain_list = domain_list
        self.overrides = overrides
        self.output_path = output_path

    def export_to_file(self):
        content = self.get_blacklist_content()
        self.write_to_file(content)

    def get_blacklist_content(self):
        content_lines = self.headers[:]
        content_lines.append('')

        content_lines.extend(self.overrides)
        content_lines.append('')

        for domain_name in self.domain_list:
            formatted_string = self.get_formatted_domain_string(domain_name)
            content_lines.append(formatted_string)

        content_lines.append('')
        return '\n'.join(content_lines)

    def get_formatted_domain_string(self, domain_name):
        justified_domain = domain_name.ljust(self.padding_size)
        return f'{justified_domain} CNAME    null.null-zone.null.'

    def write_to_file(self, content):
        file_name = self.output_path

        output_file = open(file_name, 'w+')
        output_file.write(content)
        output_file.close()

        print('Created "blacklist_file" file: "{}"'.format(file_name))


def write_to_file(domains, overrides, path):
    writer = BlacklistWriter(domains, overrides, path)
    writer.export_to_file()
