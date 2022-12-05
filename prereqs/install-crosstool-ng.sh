#!/bin/sh

# Installs crosstool-ng into ./ct-ng-bin of current system.
# Any errors telling something is missing, refer to http://crosstool-ng.github.io/docs/os-setup/
# for a guide specific to the OS (which packages would be required).

cd "${PREREQS_DIR}"
git clone https://github.com/crosstool-ng/crosstool-ng
cd crosstool-ng
./bootstrap
./configure --prefix="${PREREQS_DIR}"/ct-ng-bin --with-libintl-prefix="${PREREQS_DIR}"/ct-ng-bin
make install

# end