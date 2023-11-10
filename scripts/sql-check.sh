#!/usr/bin/env bash
set -eu -o pipefail

# https://docs.sqlfluff.com/en/stable/gettingstarted.html

sqlfluff fix --disable-progress-bar packages/schema
