import extractor


def load_domains_for_file(path):
    with open(path) as f:
        content = f.read()
    return extractor.extract_domains(content)
