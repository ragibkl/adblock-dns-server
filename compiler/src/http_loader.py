import validators
import requests

import extractor


def get_urls_from_file(path):
    with open(path) as f:
        urls = f.read().splitlines()
    urls = list(filter(validators.url, urls))
    return urls


def fetch(url):
    print('Started downloading from : {}'.format(url))
    response = requests.get(url)
    content = str(response.content, 'UTF-8')
    return content


def load_domains_for_url(url):
    content = fetch(url)
    return extractor.extract_domains(content)
