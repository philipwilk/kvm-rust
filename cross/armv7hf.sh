#!/usr/bin/env bash
mv /etc/apt/sources.list /etc/apt/sources.list.bak
mv /etc/apt/sources.d /etc/apt/sources.d.bak
echo -e "deb http://ftp.debian.org/debian sid main" > /etc/apt/sources.list
dpkg --add-architecture armhf
dpkg --add-architecture amd64
apt-get update
cd $(mktemp -d)
apt-get -d --no-install-recommends download libssl-dev:armhf
dpkg -x libssl-dev* /
rm libssl-dev*
apt-get -d --no-install-recommends download libssl-dev:amd64
mv -f /etc/apt/sources.list.bak /etc/apt/sources.list
mv -f /etc/apt/sources.d.bak /etc/apt/sources.d
dpkg --remove-architecture armhf