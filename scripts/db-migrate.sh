#!/usr/bin/env bash
set -eu -o pipefail

# https://www.postgresql.org/docs/current/app-psql.html

source .env

host="$DATABASE_URL"
name="bits"

psql "$host" --set=ON_ERROR_STOP=true \
<<SQL
\connect postgres;

drop database if exists $name with (force);
drop role if exists admin;
drop role if exists bidder;
drop role if exists seller;
drop role if exists viewer;

create database $name;
SQL

cd docs && cat schema.txt | xargs -n 1 psql "$host" --set=ON_ERROR_STOP=true --file
