import validators


def clean_whitespace(text):
    return text.replace('\r', '').replace('\t', ' ')


def is_comment(line):
    return line.startswith('#')


def is_too_short(line):
    return len(line) <= 3


def decode(domain):
    try:
        return domain.encode('idna').decode('ascii')
    except Exception as e:
        print(f'Failed to decode idna domain: {domain}')
        return None


def extract_domain(text):
    if is_comment(text):
        return None

    names = [x for x in text.split() if not is_too_short(x)]
    for name in names:
        name_idn = decode(name)
        if not name_idn:
            continue
        if '_' in name_idn:
            continue
        if validators.domain(name_idn):
            return name_idn

    return None


def extract_domains(text):
    content = clean_whitespace(text)
    domains = [ extract_domain(line) for line in content.splitlines() ]
    return filter(None, domains)


def dedup_domains(domains):
    return list(set(domains))
