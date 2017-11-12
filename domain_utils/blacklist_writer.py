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

    def __init__(self, domain_list, redirect_ip, output_path):
        self.domain_list = domain_list
        self.redirect_ip = redirect_ip
        self.output_path = output_path

    def export_to_file(self):
        content = self.get_blacklist_content()
        self.write_to_file(content)

    def get_blacklist_content(self):
        content_lines = self.headers[:]
        content_lines.append('')

        for domain_name in self.domain_list:
            formatted_string = self.get_formatted_domain_string(domain_name)
            content_lines.append(formatted_string)

        content_lines.append('')
        return '\n'.join(content_lines)

    def get_formatted_domain_string(self, domain_name):
        redirect_ip = self.redirect_ip
        formatted_string = '{domain_name} A {redirect_ip}'.format(
            domain_name=domain_name.ljust(self.padding_size),
            redirect_ip=redirect_ip
        )
        return formatted_string

    def write_to_file(self, content):
        file_name = self.output_path

        output_file = open(file_name, 'w')
        output_file.write(content)
        output_file.close()

        print('Created "blacklist_file" file: "{}"'.format(file_name))
