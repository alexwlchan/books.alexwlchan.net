#!/usr/bin/env bash

set -o errexit
set -o nounset

cd _html
python3 -m http.server 5959 &
sleep 3
open -a safari http://localhost:5959
