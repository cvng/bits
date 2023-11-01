#!/bin/zsh
set -o errexit -o nounset -o pipefail

# https://squawkhq.com/docs/cli

npx squawk-cli docs/schema.sql

# https://docs.sqlfluff.com/en/stable/gettingstarted.html

sqlfluff fix docs/schema.sql
