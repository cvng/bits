# https://www.postgresql.org/docs/current/app-psql.html

set -e

host="postgres://postgres:password@localhost:5432/bits"
name="bits"

psql "$host" --no-psqlrc --variable=ON_ERROR_STOP=1 --quiet \
<<SQL
\connect postgres;

drop database if exists $name with (force);
create database $name;
SQL

psql "$host" --no-psqlrc --variable=ON_ERROR_STOP=1 --quiet \
    --file="docs/schema.sql"
