#!/usr/bin/env bash
set -eu -o pipefail

# https://squawkhq.com/docs/cli

npx squawk-cli \
    packages/schema/src/*.sql \
    packages/schema/src/**/*.sql \
    packages/schema/src/**/**/*.sql \


# https://docs.sqlfluff.com/en/stable/gettingstarted.html

sqlfluff fix packages/schema/src
