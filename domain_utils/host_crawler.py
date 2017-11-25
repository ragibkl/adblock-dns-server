#!python3
import requests
from datetime import datetime


class HostCrawler:
    """Crawler class, to crawl the http source for the adblock list"""
    filter_list = [
        'exclude_invalid_length',
        'exclude_comments',
        'exclude_invalid_chars',
        'starts_with_local_ip',
    ]

    @classmethod
    def remove_duplicates(cls, domain_list):
        before_count = len(domain_list)
        print('Before del duplicates, row count = {}'.format(before_count))

        dedup_list = list(set(domain_list))
        after_count = len(dedup_list)
        print('After de-duplicate, row count = {}'.format(after_count))

        return dedup_list

    def __init__(self, source):
        self.source = source

    def get_domains(self):
        lines = self.fetch_list()

        domain_names = []
        for line in lines:
            domain = self.get_domain_name(line)
            if domain:
                domain_names.append(domain)

        return domain_names

    def fetch_list(self):
        print('Started downloading from : {}'.format(self.source))
        response = requests.get(self.source)
        openfile = str(response.content, 'UTF-8')
        lines = openfile.replace('\r','').split('\n')
        return lines

    def get_domain_name(self, line):
        try:
            string = line.replace('\t',' ')
            if self.is_line_valid(string):
                string_parts = string.split(' ')
                return string_parts[1]
        except Exception as e:
            print('Exception in source: "{}", for line: "{}", reason: {}'.format(self.source, line, e))

        return None

    def filter_lines(self, lines):
        filtered_lines = lines[:]

        return filtered_lines

    def is_line_valid(self, line):
        for filter_name in self.filter_list:
             if not getattr(self, filter_name)(line):
                 print('Rejected line: "{}", reason: {}'.format(line, filter_name))
                 return False

        return True

    def exclude_invalid_length(self, line):
        if len(line) == 0:
            return False

        line_parts = line.split(' ')
        return len(line_parts) >= 2

    def exclude_comments(self, line):
        if line[0] == '#':
            return False

        return True

    def exclude_invalid_chars(self, line):
        if 'localhost' in line:
            return False

        if '_' in line:
            return False

        line_parts = line.split(' ')
        if len(line_parts) > 1 and len(line_parts[1]) and line_parts[1][-1] == '.':
            return False

        return True

    def starts_with_local_ip(self, line):
        localhosts = [
            '0.0.0.0',
            '127.0.0.1',
        ]

        return any( line.startswith(host) for host in localhosts )
