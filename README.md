# Adblock DNS Server

Adblock DNS server using bind and nginx, running in Docker.

## Overview

A DNS server helps to resolve domain names on the internet into their appropriate ip addresses. It is the foundation of how the internet works. Some well known public DNS serves are as follows:

- Google:
  - 8.8.8.8
  - 8.8.4.4
- CloudFlare
  - 1.1.1.1
  - 1.0.0.1

By default, all internet connected devices and networks are configured to point at some DNS servers or other. This is usually based on the default config of the device, or network's ISP.

An Adblock DNS server works just like a regular DNS server, but with some additional custom rules to block Ads. On top of its regular functionality, an Adblock DNS server will resolve domains for known Ads services, into null webservers or an unrouteable IP address. The same custom rules can also be used to block known user tracking servers, malware sites and porn sites.

There are plenty of free publicly available Adblock DNS on the internet such as AdGuard, NextDNS, etc. I also host my own [Bancuh Adblock DNS](https://blog.bancuh.com/adblock-dns/) service for free for public use.

However, there are situations where a person might want to host their own private dns server, and have more control over its behavior. For example, you may want to host your own Adblock DNS Server as closely to your location as possible, for best performance.

This project helps you to quickly spin up your own Adblock DNS Server using a few simple commands. This project is also used to run my **Bancuh Adblock DNS** service.

The default DNS server comes preloaded with a blocklist that has been compiled from multiple sources of adblock dns host files. The sources list has been revised several times based on user feedback.

However, I also provide the tools and scripts necessary if you would like to customize your own block list.

## Requirements

To run this project, make sure your system/server has the following packages installed:

- docker
- docker-compose

## Running the server

### Quickstart

Follow these steps to get this project up and running.

1. clone this project. Then, cd into the cloned project folder.

```
git clone https://github.com/ragibkl/adblock-dns-server.git
cd adblock-dns-server
```

2. copy the sample `.env` file, and edit the values according to your server settings

```
cp sample.env .env
nano .env
```

If you are running this locally on your computer, you can leave the default values as is. This will redirect all ads traffic to your computer, and they will return empty.

If you are running this on a local network, or a public server, you should place the ipv4/ipv6 addresses of your server. This will redirect all ads to your server, with empty result.

```
# .env file - running locally
FQDN=dns.localhost.localdomain  # optional: fdqn of your server
IPV4=127.0.0.1                  # ipv4 address of your server
IPv6=::1                        # ipv6 address of your server
```

3. run the start script to start the dns server. The server may take a few minutes to spin up.

```
./start.sh
```

4. do a quick test

```shell
# tests dns lookup against Google's dns server
# should return regular/valid response
$ nslookup zedo.com 8.8.8.8
Server:		8.8.8.8
Address:	8.8.8.8#53

Non-authoritative answer:
Name:	zedo.com
Address: 64.41.197.44

# tests dns lookup against our adblock dns server
# should return our server's IP instead
$ nslookup zedo.com X.X.X.X
Server:		X.X.X.X
Address:	X.X.X.X#53

Non-authoritative answer:
Name:	zedo.com
Address: X.X.X.X
```

5. stopping the dns server

```
./stop.sh
```

### How to add additional domains to the blacklist and whitelist

The above instructions will run the `adblock-dns-server` using the precompiled adblock list. In order to add additional ads-domains to the blacklist, or filter some in a whitelist, you need to compile a custom blacklist.

1. you can modify the contents under `data/` folder. Feel free to add/remove additional domains and http sources as needed.

2. from the top of this project folder, cd into the `adblock-extra` folder.

```
cd EXAMPLES/adblock-extra
```

3. run the compile script. This will output the compiled blacklist file at `EXAMPLES/adblock-extra/data/output.d/blacklist.zone`

```
./compile.sh
```

4. the env variable config, start and stop steps are the same as before

## Configuring your device

### Short version

Option 1: WiFi Router level

- Affects all devices connected to said router.
- Instructions:
  - Go to your router admin page, under WAN settings.
  - Edit DNS settings. Use your adblock-dns server's IP address instead of Automatic or Google's (8.8.8.8, 8.8.4.4)

Option 2: Personal Computer level

- Affects single device.
- Instructions:
  - Go to your computer's network setting.
  - Change DNS settings. Use your adblock-dns server's IP address instead of Automatic or Google's (8.8.8.8, 8.8.4.4)

### Detailed tutorial

https://blog.bancuh.com/?p=71

## Contributing

This project could use some improvements and help in many areas, which includes, documentation, testing, code improvement, and deployment implementations.

If you have any suggestions, or would like to report and problem, create a Github issue to grab my attention.
