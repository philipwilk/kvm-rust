#!/usr/bin/env bash
mv /etc/apt/sources.list /etc/apt/sources.list.bak
echo -e "deb http://ftp.ports.debian.org/debian-ports/ sid main\ndeb http://ftp.debian.org/debian sid main" > /etc/apt/sources.list
dpkg --add-architecture ppc64
dpkg --add-architecture amd64
apt-get update
cd $(mktemp -d)
apt-get -d --no-install-recommends download libssl-dev:ppc64
dpkg -x libssl-dev* /
rm libssl-dev*
apt-get -d --no-install-recommends download libssl-dev:amd64
dpkg -x libssl-dev* /
mv -f /etc/apt/sources.list.bak /etc/apt/sources.list
dpkg --remove-architecture ppc64