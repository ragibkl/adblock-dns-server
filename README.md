# Adblock DNS Server

Adblock DNS Server powered by `Bancuh DNS` and `dnsdist-acme` running in Docker.

## Introduction

A DNS server helps to resolve domain names on the internet into their appropriate ip addresses.
When your web browser or apps need to fetch some info from a web service over the internet, it first makes a DNS query to find the ip address of that web service.
Next it will use that ip address to connect to the server that is hosting the service.
It is the foundation of how the internet works.

Some well known public DNS serves are as follows:

- Google:
  - 8.8.8.8
  - 8.8.4.4
- CloudFlare
  - 1.1.1.1
  - 1.0.0.1

By default, all internet connected devices and networks are configured to point at some DNS servers or other.
This is usually based on the default config of the device, or network's ISP.

An Adblock DNS server works just like a regular DNS server, but with some additional custom rules to block Ads.
On top of its regular functionality, an Adblock DNS server will resolve domains for known Ads services, into an unrouteable IP address.
Without a valid ip address, your web browser and apps cannot connect to the servers hosting the Ads service.
Ads would effectively be blocked.
The same custom rules can also be used to block known user tracking servers, malware sites and sites for adult contents.

There are plenty of free publicly available Adblock DNS on the internet such as AdGuard, NextDNS, etc.
I also host my own [Bancuh Adblock DNS](https://bancuh.com/) service for free for public use.

However, there are situations where a person might want to host their own private dns server, and have more control over its behavior.
For example, you may want to host your own Adblock DNS Server as closely to your location as possible, for best performance.

This project helps you to quickly spin up your own Adblock DNS Server using a few simple commands.
This project is also used to run my [Bancuh Adblock DNS](https://bancuh.com/) service.

The default DNS server comes preloaded with a blocklist that has been compiled from multiple sources of adblock dns host files.
The sources list has been revised several times based on user feedback.

However, I also provide the tools and scripts necessary if you would like to customize your own block list.

### Production Live Environment

This project is used to run the [Bancuh Adblock DNS](https://bancuh.com/) service.
If you want to have a taste of what you can run using this project, visit the [Quick Start](https://bancuh.com/start) page and try setting it for use on your device.

## Getting Started

### Server Requirements

To run this project, I recommend that you get a Linux VPS server with a publicly reachable static IP address.
There are plenty of providers out there, so just pick your favourite.
Using a VPS makes it easy spin up a new server with Linux installed, and to start over in case you misconfigured something.
Having a static and public IP address also allows you to share this DNS server among friends and families when you wish.

It is also possible to run this project on a local network, either using a computer running Linux, or a virtual machine.
If you are not willing to spend the money on a VPS just yet, or you are still experimenting, a virtual machine works fine.

The default dns server in this project has a blocklist of about 3 million sites.
It can take close to 2 GiB of RAM to run without a hiccup.
I recommend that you provide your server with at least that amount for a smooth operation.

### System Requirements

The server that you are preparing should run a Linux server OS.
I recommend going for the latest LTS edition of your favourite Linux distro.
I usually go for the latest Ubuntu Server LTS edition available.

You also need to be able to ssh into the server securely.
Some basic server hardening steps won't hurt as well.

Your server also needs [Docker](https://docs.docker.com/get-docker/) with the [Compose v2 plugin](https://docs.docker.com/compose/install/) (the `docker compose` command).
The old standalone `docker-compose` (Compose v1) is no longer supported; it reached end of life in 2023.

### Checking that port 53 is free

A DNS server listens for queries on TCP and UDP port 53, so nothing else on the server may be using it.
Check with:

```shell
sudo ss -lntup | grep ':53 '
# no output means port 53 is free
```

If there is no output, skip the next section.

#### If systemd-resolved is using port 53

On Ubuntu (and other distros using `systemd-resolved`), a local DNS stub listens on port 53 of `127.0.0.53`, and on Ubuntu 24.04 also `127.0.0.54`.
It blocks the DNS server from starting:

```shell
sudo ss -lntup | grep ':53 '
udp   UNCONN 0  0      127.0.0.54:53   0.0.0.0:*  users:(("systemd-resolve",pid=673,fd=16))
udp   UNCONN 0  0   127.0.0.53%lo:53   0.0.0.0:*  users:(("systemd-resolve",pid=673,fd=14))
tcp   LISTEN 0  4096 127.0.0.53%lo:53  0.0.0.0:*  users:(("systemd-resolved",pid=673,fd=15))
tcp   LISTEN 0  4096    127.0.0.54:53  0.0.0.0:*  users:(("systemd-resolve",pid=673,fd=17))
```

Turn off just the stub listener.
`systemd-resolved` keeps handling the server's own lookups, using the DNS servers your provider gives it:

```shell
sudo mkdir -p /etc/systemd/resolved.conf.d
printf '[Resolve]\nDNSStubListener=no\n' | sudo tee /etc/systemd/resolved.conf.d/adblock-dns.conf
sudo ln -sf /run/systemd/resolve/resolv.conf /etc/resolv.conf
sudo systemctl restart systemd-resolved
```

The `ln` line matters: `/etc/resolv.conf` normally points at the stub, which is now gone.
`/run/systemd/resolve/resolv.conf` lists the real upstream servers instead.

Check that port 53 is free and the server can still look up names:

```shell
sudo ss -lntup | grep ':53 '
# no output
getent hosts github.com
# prints an address
```

This survives reboots and was tested on Ubuntu 24.04.
Avoid the older advice of disabling `systemd-resolved` and editing `/etc/resolv.conf` by hand: on Ubuntu that file is a symlink, so the edit can be lost or leave the server unable to look up names after a reboot or a release upgrade.

## Running the server

Clone this project.
Then, cd into the cloned project folder.

```shell
git clone https://github.com/ragibkl/adblock-dns-server.git
cd adblock-dns-server
```

### Running the server with default config

1. cd into the `default` folder.
    Run the start script
    The server may take a few minutes to spin up.

    ```shell
    cd EXAMPLES/default
    ./start.sh
    ```

    `start.sh` returns as soon as the containers are created, so check that both stay up:

    ```shell
    docker compose ps
    ```

    If `dnsdist` keeps restarting, look at its logs with `docker compose logs dnsdist`.
    `Address in use` means something is still using port 53: see "Checking that port 53 is free" above.

2. It should also setup a `.env` file with some default values.

    ```shell
    # file: EXAMPLES/default/.env
    CONFIG_URL=https://raw.githubusercontent.com/ragibkl/adblock-dns-server/master/data/configuration.yaml
    #FORWARDERS=8.8.8.8,8.8.4.4
    TLS_ENABLED=false
    TLS_DOMAIN=dns1.example.com
    TLS_EMAIL=user@example.com
    ```

    **CONFIG_URL** - this value specifies the config file location where the server should load its configuration.
    The server uses the sources in that config file to dynamically compile the ads blocklist during server startup.
    The ads blocklist is also refreshed and recompiled every day automatically.
    The default value here points to the configuration file maintained in this repo.
    See section below on how to use a customized blocklist configuration.

    **FORWARDERS** - this value specifies custom backend dns servers to use as forwarders.
    By default, this value is empty and the server will use a built-in bind server to recursively resolve dns queries.
    This is best for privacy but might use more memory.
    If you would like to forward dns queries to other dns servers, uncomment and set this to your preferred values.
    This will also turn off the internal bind server.

    **TLS_ENABLED** - if set to `true`, the server will also enable DoH and DoT dns protocols.
    This requires that `TLS_DOMAIN` and `TLS_EMAIL` to be set correctly

    **TLS_DOMAIN** - only required when `TLS_ENABLED=true`.
    This value should point to a public domain name record that points to the public ip of your server.
    We use this value to request the TLS certificates from LetsEncrypt

    **TLS_EMAIL** - only required when `TLS_ENABLED=true`.
    This value should point to your email.
    LetsEncrypt uses this value to send you reports on expiring TLS certificates.

    If you changed any of the values above, please re-run the start script.

3. Stopping the service

```shell
./stop.sh
```

### Using a customized ads blocklist configuration

The above instructions will run the `adblock-dns-server` using the default blocklist configuration.
In order to add additional ads-domains to the blacklist, or filter some in a whitelist, you also have the option to use a custom config.

1. Modify the contents under `data/` folder as you see fit.
Feel free to add/remove additional domains and http sources as needed.

    For quick local additions without editing the main config, you can use the pre-wired placeholder files:
    - `data/blacklist.d/__extra_blacklist.hosts` — add domains to block (hosts format)
    - `data/whitelist.d/__extra_whitelist.hosts` — add domains to allow (hosts format)

2. Change the `CONFIG_URL` value to point to the local config file.
    This works because I have conveniently mounted the `data/` folder into `/local-data/` in the container.
    You can check the `docker-compose.yml` file for the corresponding line.

    ```shell
    # file: EXAMPLES/default/.env
    CONFIG_URL=/local-data/configuration.yaml
    ...
    ```

3. Rerun the start script.

### Enabling DoH and DoT protocols

By default, the server only listens for dns requests via regular dns protocol.
The server also supports listening dns requests via DoH and DoT protocols.
You can make this work by the following:

1. Your server needs to have a public IP address.
    A web request from the public Internet should be able to reach you by port 80.

2. You also need to have a domain name record that points to the public IP address of your server.
    Something in the form of `dns1.example.com`.

3. Change the config in the `.env` file as follows:

   ```shell
   # file: EXAMPLES/default/.env
   ...
   TLS_ENABLED=true             # set this to true to enable DoH and DoT
   TLS_DOMAIN=dns1.example.com  # this should be the domain name that points to the public IP address of your server
   TLS_EMAIL=user@example.com   # your email, must be a valid email address
   ```

4. Rerun the start script.

## Testing the server

Now that we have spin up our Adblock DNS Server, it is time for us to run a few tests to make sure that it is behaving as expected.

```shell
# tests dns lookup against Google's dns server
# should return regular/valid response
$ nslookup zedo.com 8.8.8.8
Server:  8.8.8.8
Address: 8.8.8.8#53

Non-authoritative answer:
Name: zedo.com
Address: 64.41.197.44

# tests dns lookup against our adblock dns server
# should return our server's IP or the null route instead
$ nslookup zedo.com X.X.X.X
Server:  X.X.X.X
Address: X.X.X.X#53

Non-authoritative answer:
Name: zedo.com
Address: X.X.X.X
```

## DNS Query Logs

You can also see the logs of all dns requests that you make to the server.

For privacy reasons, the logs viewer will only show you queries based on your current IP address.
If you make any dns queries from an IP address, you can only view those queries on a web browser from the same IP address.
Additionally, the logs file is emptied every 10 minutes.

On your web browser, simply visit the logs endpoints as follows:

- <http://server-ip-address:8080/logs>
- <http://dns1.example.com:8080/logs>
- <https://dns1.example.com:8443/logs> # only with tls enabled

## Configuring your device

### Short version

#### Option 1: WiFi Router level

This works on all devices connected to the wifi router.
This is the best approach if you want this change to apply to all devices in your network, and only have a single place to make the change.

1. Go to your router admin page, under WAN settings.

2. Edit DNS settings.

3. Use your adblock-dns server's IP address instead of automatic or Google's (8.8.8.8, 8.8.4.4).

4. Go to your router LAN -> DHCP settings.

5. Ensure that the DHCP DNS server is set to your router's IP address.

#### Option 2: Personal Computer level

This works on a single computer.

1. Go to your computer's network setting.

2. Change DNS settings.

3. Use your adblock-dns server's IP address instead of automatic or Google's (8.8.8.8, 8.8.4.4)

#### Option 3: Android Private DNS

Modern Android devices have a feature called Private DNS.
Using this feature, your device can connect to a dns server over DoT protocol.
You can use this feature if you have enabled TLS on your server.

1. On your Android device, go to Settings -> Connection & Sharing -> Private DNS.

2. Set private dns to Custom Domain, and set the domain to point to the domain of your DNS server.

### Web quick start guide

Visit <https://bancuh.com/start/> for a quickstart on how to use Bancuh DNS on your device.
However, instead of the provided servers, use the details for your own DNS servers.

## Contributing

This project could use some improvements and help in many areas, which includes, documentation, testing, code improvement, and deployment implementations.

If you have any suggestions, or would like to report a problem, create a GitHub issue to grab my attention.
