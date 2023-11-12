#!/usr/bin/env bash
set -eu -o pipefail

# https://github.com/citusdata/pg_cron

# source .env

host="postgresql://postgres:password@localhost:5432/bits"

psql "$host" --set=ON_ERROR_STOP=1 \
<<SQL
create extension pg_cron;
SQL
