#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o verbose

git pull origin live

bookish render_html
netlify deploy --prod

git push origin live
