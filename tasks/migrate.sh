# https://www.postgresql.org/docs/current/app-psql.html

set -e
source .env

host="$DATABASE_URL"

# Drop

psql "$host" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --command="\connect postgres" \
    --command="drop database if exists bits with (force);" \
    --command="create database bits;" \

# Init

psql "$host" \
    --no-psqlrc \
    --single-transaction \
    --variable=ON_ERROR_STOP=1 \
    --file="docs/schema.sql" \

# Seed

jq --compact-output ".[]" tasks/seed.json | psql "$host" \
    --no-psqlrc \
    --single-transaction \
    --variable=ON_ERROR_STOP=1 \
    --command="create table tmp (row jsonb);" \
    --command="\copy tmp (row) from stdin;" \
    --command="insert into cqrs.event (type, data) select (row->>'type')::cqrs.event_type, (row->>'data')::jsonb from tmp;" \
