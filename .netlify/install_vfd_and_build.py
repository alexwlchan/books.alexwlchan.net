#!/usr/bin/env python3

import io
import json
import os
import pprint
import subprocess
import sys
import tarfile

import httpx


if __name__ == "__main__":
    # This is a personal access token stored in Netlify as an env var
    github_token = os.environ["GITHUB_TOKEN"]

    resp = httpx.get(
        "https://api.github.com/repos/alexwlchan/books.alexwlchan.net/releases/latest",
        auth=("alexwlchan", github_token),
    )

    resp.raise_for_status()

    try:
        assets = resp.json()["assets"]
        release_tag = resp.json()["tag_name"]
    except KeyError:
        print(
            "Unable to find assets/release tag in GitHub Releases API response:",
            file=sys.stderr,
        )
        print(json.dumps(resp.json(), indent=2, sort_keys=True), file=sys.stderr)
        sys.exit(1)

    try:
        linux_asset = next(
            a for a in assets if a["name"] == "vfd-x86_64-unknown-linux-musl.tar.gz"
        )
    except IndexError:
        print(
            "Unable to find Linux binary in GitHub Releases API response:",
            file=sys.stderr,
        )
        print(json.dumps(resp.json(), indent=2, sort_keys=True), file=sys.stderr)
        sys.exit(1)

    download_url = linux_asset["url"]
    print(f"Detected download URL for {release_tag}:\n{download_url}")

    download_resp = httpx.get(
        download_url,
        auth=("alexwlchan", github_token),
        headers={"Accept": "Accept: application/octet-stream"},
        follow_redirects=True,
    )
    download_resp.raise_for_status()

    with tarfile.open(fileobj=io.BytesIO(download_resp.content), mode="r:gz") as tf:
        tf.extract(member="vfd")

    subprocess.check_call(["chmod", "+x", "vfd"])
    subprocess.check_call(["./vfd", "build"])
