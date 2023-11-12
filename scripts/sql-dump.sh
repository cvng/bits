#!/usr/bin/env bash
set -eu -o pipefail

# https://www.postgresql.org/docs/current/app-pgdump.html

source .env

host="postgres://postgres:password@localhost:5432/bits"

pg_dump "$host" --schema-only --no-owner --format=plain --file=docs/schema.sql \
    --exclude-schema=cron
