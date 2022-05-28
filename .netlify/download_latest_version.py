#!/usr/bin/env python3

import json
import time
import urllib.request
from pprint import pprint


ONE_MINUTE = 60

RELEASE_URL = (
    "https://api.github.com/repos/alexwlchan/books.alexwlchan.net/releases/latest"
)


def get_asset_download_url():
    """
    Fetch the asset URL using the GitHub Releases API.

    For the purposes of this script, these are the interesting bits of the
    API response that we want to pay attention to:

        {
          "assets": [
            {
              "name": "vfd-x86_64-apple-darwin.tar.gz",
              "url": "https://api.github.com/repos/alexwlchan/books.alexwlchan.net/releases/assets/64229966",
              ...
            },
            {
              "name": "vfd-x86_64-pc-windows-msvc.zip",
              "url": "https://api.github.com/repos/alexwlchan/books.alexwlchan.net/releases/assets/64229889",
              ...
            },
            {
              "name": "vfd-x86_64-unknown-linux-gnu.tar.gz",
              "url": "https://api.github.com/repos/alexwlchan/books.alexwlchan.net/releases/assets/64229611",
              ...
            }
          ],
        }

    See https://docs.github.com/en/rest/releases/releases#get-the-latest-release
    """
    start = time.time()

    # When I publish a new version, GitHub Actions will create a new release
    # that Netlify will try to pull from -- but it will discover there aren't
    # any binaries yet.
    #
    # It should wait about ten minutes for binaries to become available -- long
    # enough that it should always succeed, but not so long it uses all my
    # Netlify build minutes waiting for GitHub Actions.
    while time.time() - start < 10 * ONE_MINUTE:
        with urllib.request.urlopen(RELEASE_URL) as resp:
            release = json.load(resp)

        assets = release["assets"]

        if not assets:
            print("Latest release has no assets, waiting...")
            time.sleep(5)
            continue

        try:
            linux_asset = next(
                ast
                for ast in assets
                if ast["name"] == "vfd-x86_64-unknown-linux-gnu.tar.gz"
            )
        except StopIteration:
            print("Latest release has no Linux asset, waiting...")
            time.sleep(5)
            continue

        return linux_asset["url"]

    raise RuntimeError("Waited 10 minutes for Linux release, none available!")


def download_asset(url):
    """
    Download the asset using the GitHub Release Assets API.

    We supply the headers required by the GitHub API.

    See https://docs.github.com/en/rest/releases/assets#get-a-release-asset
    """
    # See  https://docs.github.com/en/rest/releases/assets#get-a-release-asset
    opener = urllib.request.build_opener()
    opener.addheaders = [("Accept", "application/octet-stream")]
    urllib.request.install_opener(opener)
    urllib.request.urlretrieve(url, "vfd-x86_64-unknown-linux-gnu.tar.gz")


if __name__ == "__main__":
    opener = urllib.request.build_opener()
    opener.addheaders = [("User-Agent", "alexwlchan")]
    urllib.request.install_opener(opener)

    download_url = get_asset_download_url()
    download_asset(download_url)
