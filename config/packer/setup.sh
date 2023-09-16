#!/bin/bash

echo "Install git"
yum install -y git

echo "install github cli"
curl https://raw.githubusercontent.com/dvershinin/apt-get-centos/master/apt-get.sh -o /usr/local/bin/apt-get
chmod 0755 /usr/local/bin/apt-get
apt update
apt install gh

echo "Install Docker engine"
yum update -y
yum install docker -y
usermod -aG docker ec2-user
usermod -aG docker exch
chmod 666 /var/run/docker.sock 
systemctl enable docker

echo "Setup SSH key"
mkdir /var/lib/exch/.ssh
touch /var/lib/exch/.ssh/known_hosts
chown -R exch:exch /var/lib/exch/.ssh
chmod 700 /var/lib/exch/.ssh
mv /tmp/id_rsa /var/lib/exch/.ssh/id_rsa
chmod 600 /var/lib/exch/.ssh/id_rsa
chown -R exch:exch /var/lib/exch/.ssh/id_rsa

