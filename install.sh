#!/bin/bash

set -e

REPO="lhalf/redacta"
BIN="redacta"
INSTALL_DIR="/usr/local/bin"

VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")')

if [[ -z "${VERSION}" ]]; then
    echo "failed to detect latest release version"
    exit 1
fi

curl -sL "https://github.com/${REPO}/releases/download/${VERSION}/${BIN}.tar.gz" | tar xz

chmod +x ${BIN}
sudo mv ${BIN} ${INSTALL_DIR}

$BIN --help
