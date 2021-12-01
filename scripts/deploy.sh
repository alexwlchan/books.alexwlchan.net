#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o verbose

git pull origin development

rm -rf _html

python3.8 scripts/render_html.py

rsync --archive --verbose --compress --delete \
  _html/ alexwlchan@helene.linode:sites/books.alexwlchan.net

ssh alexwlchan@helene.linode 'chmod 644 ~/sites/books.alexwlchan.net/static/*'
ssh alexwlchan@helene.linode 'chmod 644 ~/sites/books.alexwlchan.net/covers/*'
ssh alexwlchan@helene.linode 'chmod 644 ~/sites/books.alexwlchan.net/thumbnails/*'
ssh alexwlchan@helene.linode 'chmod 644 ~/sites/books.alexwlchan.net/**/*.html'

git push origin
