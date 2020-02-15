import os
import extractor

def get_paths_in_dir(dir):
    paths = os.listdir(dir)
    paths = [ os.path.join(dir, path) for path in paths ]
    print(paths)
    return paths


def load_domains_for_file(path):
    with open(path) as f:
        content = f.read()
    return extractor.extract_domains(content)


def load_overrides_for_file(path):
    with open(path) as f:
        content = f.read().splitlines()
    return content
