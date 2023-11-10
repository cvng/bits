#!/usr/bin/env bash
set -eu -o pipefail

# https://www.postgresql.org/docs/current/app-pgdump.html

source .env

host="$DATABASE_URL"

pg_dump "$host" --schema-only --no-owner --format=plain --file=docs/schema.sql
