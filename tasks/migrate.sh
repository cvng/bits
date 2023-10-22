# https://www.postgresql.org/docs/current/app-psql.html

set -e

host="postgres://postgres:password@localhost:5432/bits"
name="bits"

psql "$host" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --command="\connect postgres;" \
    --command="drop database if exists $name with (force);" \
    --command="create database $name;" \
    --command="\connect $name;" \
    --file="docs/schema.sql" \
