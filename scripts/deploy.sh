#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o verbose

git pull origin live

bookish render_html
cp _redirects _html/_redirects
netlify deploy --prod

git push origin live
