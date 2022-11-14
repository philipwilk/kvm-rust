#!/usr/bin/env bash
mv /etc/apt/sources.list /etc/apt/sources.list.bak
echo -e "deb http://ftp.debian.org/debian sid main\ndeb http://ftp.debian.org/debian sid main" > /etc/apt/sources.list
dpkg --add-architecture armhf
dpkg --add-architecture amd64
apt-get update
cd $(mktemp -d)
apt-get -d --no-install-recommends download libssl-dev:armhf
dpkg -x libssl-dev* /
rm libssl-dev*
apt-get -d --no-install-recommends download libssl-dev:amd64
dpkg -x libssl-dev* /
mv -f /etc/apt/sources.list.bak /etc/apt/sources.list
dpkg --remove-architecture armhf