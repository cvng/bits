# https://www.postgresql.org/docs/current/app-psql.html

set -e
source .env

host="$DB_HOST"
name="$DB_NAME"

psql "$host" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --command="drop database if exists $name with (force);" \
    --command="create database $name;" \

psql "$host/$name" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --single-transaction \
    --file="docs/schema.sql" \

PGOPTIONS='--client-min-messages=warning' psql "$host/$name" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --file="scripts/seed.sql" \
