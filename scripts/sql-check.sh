#!/usr/bin/env bash
set -eu -o pipefail

# https://squawkhq.com/docs/cli

npx squawk-cli \
    packages/schema/sql/*.sql \
    packages/schema/sql/**/*.sql \
    packages/schema/sql/**/include/*.sql \


# https://docs.sqlfluff.com/en/stable/gettingstarted.html

sqlfluff fix packages/schema/sql
