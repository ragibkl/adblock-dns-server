#!/usr/bin/env python3
# Usage: ./check.py <domain> [qtype]
# Examples:
#   ./check.py google.com           # A and AAAA records (default)
#   ./check.py google.com CNAME     # specific record type
#   ./check.py zedo.com             # should return 0.0.0.0 (blocked)

import base64
import random
import socket
import ssl
import struct
import sys
import urllib.request

SERVERS = [
    "sg-dns1.bancuh.com",
    "sg-dns2.bancuh.com",
    "fr-dns1.bancuh.com",
    "fr-dns2.bancuh.com",
    "jp-dns1.bancuh.com",
    "jp-dns2.bancuh.com",
    "us-dns1.bancuh.com",
]

QTYPES = {"A": 1, "AAAA": 28, "MX": 15, "CNAME": 5, "TXT": 16}


def build_query(domain, qtype):
    msg_id = random.randint(0, 0xFFFF)
    header = struct.pack("!HHHHHH", msg_id, 0x0100, 1, 0, 0, 0)
    question = b"".join(struct.pack("B", len(l)) + l.encode() for l in domain.split("."))
    question += b"\x00" + struct.pack("!HH", qtype, 1)
    return header + question


def read_name(data, i):
    labels = []
    while True:
        length = data[i]
        if length == 0:
            i += 1
            break
        if length & 0xC0 == 0xC0:  # pointer
            ptr = struct.unpack("!H", data[i : i + 2])[0] & 0x3FFF
            labels.append(read_name(data, ptr)[0])
            i += 2
            break
        labels.append(data[i + 1 : i + 1 + length].decode())
        i += 1 + length
    return ".".join(labels), i


def parse_response(data):
    if len(data) < 12:
        return "(no response)"

    _, flags, qdcount, ancount, *_ = struct.unpack("!HHHHHH", data[:12])
    rcode = flags & 0xF

    rcodes = {1: "FORMERR", 2: "SERVFAIL", 3: "NXDOMAIN", 5: "REFUSED"}
    if rcode != 0:
        return f"RCODE: {rcodes.get(rcode, rcode)}"

    i = 12
    for _ in range(qdcount):
        while data[i] != 0:
            i += data[i] + 1
        i += 5  # null + type + class

    answers = []
    for _ in range(ancount):
        _, i = read_name(data, i)
        rtype, _, _, rdlength = struct.unpack("!HHIH", data[i : i + 10])
        i += 10
        rdata = data[i : i + rdlength]
        i += rdlength

        if rtype == 1:
            answers.append(f"A     {socket.inet_ntop(socket.AF_INET, rdata)}")
        elif rtype == 28:
            answers.append(f"AAAA  {socket.inet_ntop(socket.AF_INET6, rdata)}")
        elif rtype == 5:
            cname, _ = read_name(data, i - rdlength)
            answers.append(f"CNAME {cname}")
        else:
            answers.append(f"type={rtype} rdata={rdata.hex()}")

    return "\n    ".join(answers) if answers else "(no answers)"


def query_doh(server, query):
    dns_param = base64.urlsafe_b64encode(query).rstrip(b"=").decode()
    url = f"https://{server}/dns-query?dns={dns_param}"
    req = urllib.request.Request(url, headers={"accept": "application/dns-message"})
    with urllib.request.urlopen(req, timeout=5) as resp:
        return parse_response(resp.read())


def query_dot(server, query):
    ctx = ssl.create_default_context()
    with socket.create_connection((server, 853), timeout=5) as sock:
        with ctx.wrap_socket(sock, server_hostname=server) as tls:
            # DoT uses 2-byte length prefix
            tls.sendall(struct.pack("!H", len(query)) + query)
            length = struct.unpack("!H", tls.recv(2))[0]
            return parse_response(tls.recv(length))


def query_dns(server, query):
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
        sock.settimeout(5)
        sock.sendto(query, (server, 53))
        data, _ = sock.recvfrom(4096)
        return parse_response(data)


def query_server(server, domain, qtypes):
    print(f"==> {server}: {domain}")
    for label, fn in [("DoH", query_doh), ("DoT", query_dot), ("DNS", query_dns)]:
        for qtype_name, qtype in qtypes:
            query = build_query(domain, qtype)
            try:
                result = fn(server, query)
            except Exception as e:
                result = f"ERROR: {e}"
            print(f"  {label:3}  {qtype_name:5}  {result}")
    print()


def main():
    domain = sys.argv[1] if len(sys.argv) > 1 else "google.com"

    if len(sys.argv) > 2:
        qtype_name = sys.argv[2].upper()
        qtypes = [(qtype_name, QTYPES.get(qtype_name, 1))]
    else:
        qtypes = [("A", 1), ("AAAA", 28)]

    for server in SERVERS:
        query_server(server, domain, qtypes)


if __name__ == "__main__":
    main()
