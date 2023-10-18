# https://www.postgresql.org/docs/current/app-psql.html

set -e
source .env

host="$DB_HOST"
name="$DB_NAME"

time psql "$host" \
    --command="drop database if exists $name;" \
    --command="create database $name;" \
    --command="\connect $name" \
    --file="docs/schema.sql" \
    --set ON_ERROR_STOP=1
