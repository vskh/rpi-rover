#!/bin/sh

# Installing tools required for building the project.
# Attempts to guess what's needed to cross-compile Rust into ARM.
# Follows either of two ways:
# - if on Linux, installing docker and cross is enough. Cross will handle toolchains;
# - otherwise, use crosstool-ng which is more generic way but requires a bit more work to get setup.

# for docker-based cross-compilation
cargo install cross

# for WASM front-end
cargo install wasm-pack

# required by wasm-pack now
# just build it from source as wasm-pack does not do it by itself, and prebuilt binary is not available for every platform
cargo install wasm-opt

if [ "$(uname -s)" != "Linux" ];
then
  "${PREREQS_DIR}"/install-crosstool-ng.sh
  "${PREREQS_DIR}"/install-toolschain.sh
fi

# end