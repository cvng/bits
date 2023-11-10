#!/usr/bin/env bash
set -eu -o pipefail

# https://squawkhq.com/docs/cli

npx squawk-cli $(find packages/schema/src -name '*.sql' ! -name 'lib.sql')

# https://docs.sqlfluff.com/en/stable/gettingstarted.html

sqlfluff fix --quiet --disable-progress-bar packages/schema/src
