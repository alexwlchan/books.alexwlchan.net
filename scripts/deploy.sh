#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o verbose

git pull origin live

bookish render_html
netlify deploy --dir=_html/ --prod

git push origin live
