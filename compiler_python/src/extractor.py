import validators
from timing_utils import timing

MIN_LENGTH = 5
MAX_LENGTH = 240


def clean_comment(text):
    index = text.find('#')
    if index == -1:
        return text
    return text[:index].strip()


def clean_whitespace(text):
    return text.replace('\r', '').replace('\t', ' ')


def is_correct_length(line):
    length = len(line)
    if length < MIN_LENGTH or length > MAX_LENGTH:
        return False
    return True


def is_ip_address(text):
    if validators.ip_address.ipv4(text):
        return True
    return validators.ip_address.ipv6(text)


def decode(domain):
    try:
        return domain.encode('idna').decode('ascii')
    except Exception as e:
        print(f'Failed to decode idna domain: {domain}')
        return None


def extract_domain(text):
    text = clean_comment(text)

    names = [x for x in text.split() if is_correct_length(x)]
    for name in names:
        if is_ip_address(name):
            continue

        name_idn = decode(name)
        if not name_idn:
            continue
        if '_' in name_idn:
            continue
        if validators.domain(name_idn):
            return name_idn

    return None


@timing
def extract_domains(text):
    content = clean_whitespace(text)
    domains = [ extract_domain(line) for line in content.splitlines() ]
    return filter(None, domains)


@timing
def dedup_domains(domains):
    return list(set(domains))


@timing
def exclude_whitelist_domains(domains, whitelist):
    exclusion_list = set([decode(d) for d in whitelist])
    return [d for d in domains if d not in exclusion_list]


@timing
def sort_domains(domains):
    def get_sort_key(domain):
        return '.'.join(reversed(domain.split('.')))
    return sorted(domains, key=get_sort_key)
