#!/bin/bash

echo "Install git"
yum install -y git



echo "Install c++ compiler"
yum install gcc-c++
yum install cmake

echo "Setup SSH key"
mkdir -p /var/lib/exch/.ssh
touch /var/lib/exch/.ssh/known_hosts
# Check if the 'exch' user exists; if not, create it
if ! id -u exch &>/dev/null; then
  sudo useradd -m -s /bin/bash exch
fi
chown -R exch:exch /var/lib/exch/.ssh
chmod 700 /var/lib/exch/.ssh
mv /tmp/id_rsa /var/lib/exch/.ssh/id_rsa
chmod 600 /var/lib/exch/.ssh/id_rsa
chown -R exch:exch /var/lib/exch/.ssh/id_rsa