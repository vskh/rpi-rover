#!/bin/sh

# This is sample script to prepare the build/packaging environment for the project.
# This is provided for guidance mainly and might need to be adjusted on some systems.
#
# It assumes certain environment to be present before setting up this project:
# - curl is installed (used by scripted docker install; not required if docker is already there);
# - python/pip is installed (used by docker-compose install; not required if docker-compose is
#   already there);
# - GNU Compiler Collection/misc build tools are installed (used by crosstool-ng to build a compiler
#   toolchain for ARM to be able to target Raspberry Pi; not required if building on Raspberry Pi
#   directly).
#

SCRIPT_DIR=`dirname $0`
export PREREQS_DIR=`readlink -f $SCRIPT_DIR`

if [ "`uname -s`" = "Linux" ];
then
  ${PREREQS_DIR}/install-docker.sh
fi

${PREREQS_DIR}/install-tools.sh

# end
