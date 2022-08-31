#!/usr/bin/env bash
mv /etc/apt/sources.list /etc/apt/sources.list.bak
mv /etc/apt/sources.d /etc/apt/sources.d.bak
echo -e "deb http://archive.debian.org/debian jessie main\ndeb http://archive.debian.org/debian jessie-backports main\ndeb http://ftp.ports.debian.org/debian-ports unstable main\ndeb http://ftp.ports.debian.org/debian-ports unreleased main" > /etc/apt/sources.list
dpkg --add-architecture ppc64
apt-get update
cd $(mktemp -d)
apt-get -d --no-install-recommends download libssl-dev:ppc64
dpkg -x libssl-dev* /
mv -f /etc/apt/sources.list.bak /etc/apt/sources.list
mv -f /etc/apt/sources.d.bak /etc/apt/sources.d
dpkg --remove-architecture ppc64