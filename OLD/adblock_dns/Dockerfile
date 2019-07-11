FROM ubuntu:14.04

RUN apt-get update
RUN apt-get install -y bind9

COPY config/named.conf.local /etc/bind/named.conf.local
COPY config/named.conf.options /etc/bind/named.conf.options

# COPY config/badlist /etc/bind/badlist
# COPY config/null.zone.file /etc/bind/null.zone.file
# RUN chown root:bind /etc/bind/named.conf.local /etc/bind/named.conf.options /etc/bind/badlist /etc/bind/null.zone.file
# RUN chmod a-rwx    /etc/bind/named.conf.local /etc/bind/named.conf.options /etc/bind/badlist /etc/bind/null.zone.file
# RUN chmod ug+rw    /etc/bind/named.conf.local /etc/bind/named.conf.options /etc/bind/badlist /etc/bind/null.zone.file

RUN chown root:bind /etc/bind/named.conf.local /etc/bind/named.conf.options
RUN chmod a-rwx /etc/bind/named.conf.local /etc/bind/named.conf.options
RUN chmod ug+rw /etc/bind/named.conf.local /etc/bind/named.conf.options
EXPOSE 53
CMD named -g
