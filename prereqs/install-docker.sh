#!/bin/sh

# Installs docker on *nix using script provided by Docker.
#
# Note: this might modify the system because it usually uses OS package manager thus
# *Requires ROOT privileges*.

curl -sSL https://get.docker.com | sh
#curl -L "https://github.com/docker/compose/releases/download/1.23.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
apt install python3-pip

# end