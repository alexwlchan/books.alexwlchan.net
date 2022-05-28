#!/usr/bin/env bash
# This is the script that runs on Netlify to build the site.

set -o errexit
set -o nounset

DOWNLOAD_URL=$(curl --silent 'https://api.github.com/repos/alexwlchan/books.alexwlchan.net/releases/latest' \
  | jq -r ' .assets | map(.browser_download_url) | map(select(test(".*linux.*")))[0]'
)

# The --location flag means we follow redirects
curl --location "$DOWNLOAD_URL" > ~/.cargo/bin/vfd.tar.gz
tar -xzf ~/.cargo/bin/vfd.tar.gz
chmod +x vfd

./vfd build
