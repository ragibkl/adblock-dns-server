import validators
import requests
from requests_futures.sessions import FuturesSession

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
    print('Completed downloading from : {}'.format(url))
    return content


def fetch_urls(urls):
    contents = []
    for url in urls:
        content = fetch(url)
        contents.append(content)
    return contents


def fetch_urls_parallel(urls):
    session = FuturesSession()
    futures = [ session.get(url) for url in urls ]
    contents = []
    for f in futures:
        response = f.result()
        print('Completed downloading from : {}'.format(response.url))
        content = str(response.content, 'UTF-8')
        contents.append(content)

    return contents


def load_domains_for_urls(urls):
    contents = fetch_urls(urls)
    res_domains = []
    for c in contents:
        domains = extractor.extract_domains(c)
        res_domains.extend(domains)

    return res_domains
