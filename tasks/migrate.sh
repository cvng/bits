# https://www.postgresql.org/docs/current/app-psql.html

set -e

# Drop

psql "postgres://postgres:password@localhost:5432/postgres" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --command="\connect postgres;" \
    --command="drop database if exists bits with (force);" \
    --command="create database bits;" \
    --quiet

# Init

psql "postgres://postgres:password@localhost:5432/bits" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --command="\connect bits;" \
    --file="docs/schema.sql" \
    --quiet
