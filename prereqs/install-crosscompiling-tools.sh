#!/bin/sh

# Attempts to guess what's needed to cross-compile Rust into ARM.
# Follows either of two ways:
# - if on Linux, installing docker and cross is enough. Cross will handle toolchains;
# - otherwise, use crosstool-ng which is more generic way but requires a bit more work to get setup.

cargo install cross

if [ "`uname -s`" != "Linux" ];
then
  ${PREREQS_DIR}/install-crosstool-ng.sh
  ${PREREQS_DIR}/install-toolschain.sh
fi

# end