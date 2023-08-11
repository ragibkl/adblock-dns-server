#!/usr/bin/env bash

# SG
curl -H 'accept: application/dns-message' 'https://sg-dns1.bancuh.com/dns-query?dns=rmUBAAABAAAAAAAAB2NhcmVlcnMHb3BlbmRucwNjb20AAAEAAQ' | hexdump -C
curl -H 'accept: application/dns-message' 'https://sg-dns2.bancuh.com/dns-query?dns=rmUBAAABAAAAAAAAB2NhcmVlcnMHb3BlbmRucwNjb20AAAEAAQ' | hexdump -C

# FR
curl -H 'accept: application/dns-message' 'https://fr-dns1.bancuh.com/dns-query?dns=rmUBAAABAAAAAAAAB2NhcmVlcnMHb3BlbmRucwNjb20AAAEAAQ' | hexdump -C
curl -H 'accept: application/dns-message' 'https://fr-dns2.bancuh.com/dns-query?dns=rmUBAAABAAAAAAAAB2NhcmVlcnMHb3BlbmRucwNjb20AAAEAAQ' | hexdump -C

# JP
curl -H 'accept: application/dns-message' 'https://jp-dns1.bancuh.com/dns-query?dns=rmUBAAABAAAAAAAAB2NhcmVlcnMHb3BlbmRucwNjb20AAAEAAQ' | hexdump -C
