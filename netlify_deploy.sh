#!/usr/bin/env bash
# This is the script that runs on Netlify to build the site.

set -o errexit
set -o nounset

# When I publish a new version, GitHub Actions will create a new release
# that Netlify will try to pull from -- but it will discover there aren't
# any binaries yet.
#
# It should wait about ten minutes for binaries to become available -- long
# enough that it should always succeed, but not so long it uses all my
# Netlify build minutes waiting for GitHub Actions.
for i in {1,..,600}
do

  ASSETS=$(curl --silent 'https://api.github.com/repos/alexwlchan/books.alexwlchan.net/releases/latest' \
    | jq -r ' .assets')

  if [[ "$ASSETS" == '[]' ]]
  then
    echo "No assets available yet, waiting..."
    sleep 1
    continue
  fi

  DOWNLOAD_URL=$(
    echo "$ASSETS" | jq -r 'map(.browser_download_url) | map(select(test(".*linux.*")))[0]'
  )

  if [[ "$DOWNLOAD_URL" != "null" ]]
  then
    break
  else
    echo "No binaries available yet, waiting..."
    sleep 1
  fi
done

# The --location flag means we follow redirects
curl --location "$DOWNLOAD_URL" > ~/.cargo/bin/vfd.tar.gz
tar -xzf ~/.cargo/bin/vfd.tar.gz
chmod +x vfd

./vfd build
