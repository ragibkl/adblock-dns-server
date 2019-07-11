import validators


def clean_whitespace(text):
    return text.replace('\r', '').replace('\t', ' ')


def is_comment(line):
    return line.startswith('#')


def extract_domain(text):
    if is_comment(text):
        return None

    for name in text.split():
        name_idn = name.encode('idna').decode('ascii')
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
    pass
