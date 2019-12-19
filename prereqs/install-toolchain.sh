#!/bin/sh

# Installs and configures Rust toolchain for cross-compilation to ARM platform.

CT_NG_BIN=${PREREQS_DIR}/ct-ng-bin
PATH=${CT_NG_BIN}:${PATH}

cd ${PREREQS_DIR}
mkdir x-tools
cd x-tools
ct-ng armv7-rpi2-linux-gnueabihf
ct-ng build

# Configure Rust
rustup target add armv7-unknown-linux-gnueabihf

cat <<EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "${PREREQS_DIR}/x-tools/armv7-rpi2-linux-gnueabihf/bin/armv7-rpi2-linux-gnueabihf-gcc"
EOF >> ${HOME}/.cargo/config

# end