# Adblock DNS Server

Adblock DNS server using bind and nginx, running in Docker.

## Introduction

A DNS server helps to resolve domain names on the internet into their appropriate ip addresses. When your web browser or apps need to fetch some info from a web service over the internet, it first makes a DNS query to find the ip address of that web service. Next it will use that ip address to connect to the server that is hosting the service. It is the foundation of how the internet works.

Some well known public DNS serves are as follows:

- Google:
  - 8.8.8.8
  - 8.8.4.4
- CloudFlare
  - 1.1.1.1
  - 1.0.0.1

By default, all internet connected devices and networks are configured to point at some DNS servers or other. This is usually based on the default config of the device, or network's ISP.

An Adblock DNS server works just like a regular DNS server, but with some additional custom rules to block Ads. On top of its regular functionality, an Adblock DNS server will resolve domains for known Ads services, into null webservers or an unrouteable IP address. Without a valid ip address, your web browser and apps cannot connect to the servers hosting the Ads service. Ads would effectively be blocked. The same custom rules can also be used to block known user tracking servers, malware sites and sites for adult contents.

There are plenty of free publicly available Adblock DNS on the internet such as AdGuard, NextDNS, etc. I also host my own [Bancuh Adblock DNS](https://blog.bancuh.com/adblock-dns/) service for free for public use.

However, there are situations where a person might want to host their own private dns server, and have more control over its behavior. For example, you may want to host your own Adblock DNS Server as closely to your location as possible, for best performance.

This project helps you to quickly spin up your own Adblock DNS Server using a few simple commands. This project is also used to run my **Bancuh Adblock DNS** service.

The default DNS server comes preloaded with a blocklist that has been compiled from multiple sources of adblock dns host files. The sources list has been revised several times based on user feedback.

However, I also provide the tools and scripts necessary if you would like to customize your own block list.

## Getting Started

### Server Requirements

To run this project, I recommend that you get a Linux VPS server with a publicly reachable static IP address. There are plenty of providers out there, so just pick your favourite. Using a VPS makes it easy spin up a new server with Linux installed, and to start over in case you misconfigured something. Having a static and public IP address also allows you to share this DNS server among friends and families when you wish.

It is also possible to run this project on a local network, either using a computer running Linux, or a virtual machine. If you are not willing to spend the money on a VPS just yet, or you are still experimenting, a virtual machine works fine.

The default dns server in this project has a blocklist of about 1.5 million sites. It can take close to 2 GiB of RAM to run without a hiccup. I recommend that you provide your server with at least that amount for a smooth operation.

### System Requirements

The server that you are preparing should run a Linux server OS. I recommend going for the latest LTS edition of your favourite Linux distro. I usually go for the latest Ubuntu Server LTS edition available.

You also need to be able to ssh into the server securely. Some basic server hardening steps won't hurt as well.

Your server also needs to have [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/) installed.

### Checking if the network port is available

A DNS server typically listens and handles DNS queries on TCP/UDP ports 53. Before proceeding any further, we need to ensure that the ports that we need are available for use, and not being occupied by a different process running on the server.

Run the following commands on your server as root.

```shell
sudo lsof -i:53
# no result
```

If you see an empty prompt, it means that the port is open. Feel free to skip the next section.

#### Disabling systemd-resolve

On some modern Linux distros running `systemd`, there is a locally running dns service called `systemd-resolved` that listens on port `53`. By default, this service handles any outgoing DNS results and caches them locally for performance.

```shell
sudo lsof -i:53
COMMAND   PID            USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
systemd-r 967 systemd-resolve   16u  IPv4  34553      0t0  UDP localhost:domain
systemd-r 967 systemd-resolve   17u  IPv4  34554      0t0  TCP localhost:domain (LISTEN)
```

If you see something similar to the above prompt it means that `systemd-resolved` is running and needs to be disabled. There are plenty of guides on the internet on how to do this for specific Linux distros. The following should work for `Ubuntu Server 20.04`.

1. Disable and stop the systemd-resolved service:

```shell
sudo systemctl disable systemd-resolved
sudo systemctl stop systemd-resolved
```

2. Edit the file: `/etc/resolv.conf`, maybe replacing with google dns

```shell
nameserver 8.8.8.8
nameserver 8.8.4.4
```

3. Test that the port is open

```shell
sudo lsof -i:53
# no result
```

## Running the server

### Downloading this project

Clone this project. Then, cd into the cloned project folder.

```shell
git clone https://github.com/ragibkl/adblock-dns-server.git
cd adblock-dns-server
```

### Running with preloaded blocklist

1. cd into the `default` folder. Run the start script. The server may take a few minutes to spin up.

```shell
cd EXAMPLES/adblock-default
./start.sh
```

2. It should also setup a `.env` file with some default values.

```shell
# file: EXAMPLES/adblock-default/.env
FQDN=dns.localhost.localdomain
IPV4=0.0.0.0
IPv6=::
FORWARDER_1=8.8.8.8
FORWARDER_2=8.8.4.4
```

**IPV4/IPV6** - the default values here will route ads to null unreachable IPs. If you change them to the IP addresses of your servers, it will instead redirect to the default http server, which will show a tasteful blocked page.

**FORWARDER_1/2** - the default values here will direct regular dns requests to Google's DNS servers. You can change them to your own favourite DNS providers.

If you changed any of the values above, please re-run the start script.

3. Test that your setup works. See the [testing](#testing-the-server) section below.

4. Stopping the service

```shell
./stop.sh
```

### Running with a customized blocklist

The above instructions will run the `adblock-dns-server` using the precompiled adblock list. In order to add additional ads-domains to the blacklist, or filter some in a whitelist, you need to compile a custom blocklist.

1. you can modify the contents under `data/` folder. Feel free to add/remove additional domains and http sources as needed.

2. from the top of this project folder, cd into the `adblock-extra` folder.

```
cd EXAMPLES/adblock-extra
```

3. run the compile script. This will build a custom adblock_dns image with a custom-compiled blacklist, based on the contents under `data/` folder.

```
./compile.sh
```

4. the env variable config, start and stop steps are the same as before

## Testing the server

Now that we have spin up our Adblock DNS Server, it is time for us to run a few tests to make sure that it is behaving as expected.

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
