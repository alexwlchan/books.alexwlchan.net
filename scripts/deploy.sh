#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o verbose

git pull origin live

find _html -name '*.html' -delete

bookish render_html

rsync --archive --verbose --compress --delete \
  _html/ alexwlchan@helene.linode:sites/books.alexwlchan.net

ssh alexwlchan@helene.linode 'chmod 644 ~/sites/books.alexwlchan.net/static/*'
ssh alexwlchan@helene.linode 'chmod 644 ~/sites/books.alexwlchan.net/thumbnails/*'
ssh alexwlchan@helene.linode 'chmod 644 ~/sites/books.alexwlchan.net/**/*.html'

git push origin live
