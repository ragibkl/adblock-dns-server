# Adblock DNS Server
Adblocking DNS server using bind and nginx services. The services are run inside docker containers.

## Overview
This project is created to help block Ads, at the DNS resolution level.
Using this project, you can quickly bring up a caching DNS server that also redirects Ads to a null webserver.
If you this DNS server on your devices/wifi-router instead of your ISP's or other regular DNS servers (Google, OpenDNS), Ads wil be blocked.

## Requirements
To run this project, make sure your system/server has the following packages installed: 
- python3
- docker
- docker-compose

## Running the server
Follow these steps to get this project up and running.
1. Clone this project   
   `git clone https://github.com/ragibkl/adblock-dns-server.git`

2. cd into the cloned project.  
   `cd adblock-dns-server`   

3. run the start script    
   `./start.sh`

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
    `./stop.sh`

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
