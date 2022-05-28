#!/usr/bin/env bash
# This is the script that runs on Netlify to build the site.

set -o errexit
set -o nounset

python3 .netlify/download_latest_version.py
tar -xzf ~/.cargo/bin/vfd-x86_64-unknown-linux-gnu.tar.gz
chmod +x vfd

./vfd build
