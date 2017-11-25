#!python3
import requests
from datetime import datetime


# hosts file line filter methods
def is_not_comment(line):
    if line.startswith('#'):
        return False
    return True


def isvalid_length(line):
    if len(line) == 0:
        return False
    line_parts = line.split(' ')

    return len(line_parts) >= 2


def does_not_contains_invalid_chars(line):
    invalid_strings = [
        'localhost',
        '_',
    ]
    if any( s in line for s in invalid_strings ):
        return False

    return True


def starts_with_local_ip(line):
    localhosts = [
        '0.0.0.0 ',
        '127.0.0.1 ',
    ]

    return any( line.startswith(host) for host in localhosts )


class HostCrawler:
    """Crawler class, to crawl the http source for the adblock list"""
    filters = [
        is_not_comment,
        isvalid_length,
        does_not_contains_invalid_chars,
        starts_with_local_ip,
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

    def is_line_valid(self, line):
        for filter_func in self.filters:
            if not filter_func(line):
                print('Rejected line: "{}", reason: {}'.format(line, filter_func.__name__))
                return False

        return True
