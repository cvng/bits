#!/usr/bin/env bash
set -eu -o pipefail

# https://www.postgresql.org/docs/current/app-psql.html

source .env

host="postgresql://postgres:password@localhost:5432/bits"
name="bits"

psql "$host" --set=ON_ERROR_STOP=true \
<<SQL
\connect postgres;

drop database if exists $name with (force);
drop role if exists admin;
drop role if exists buyer;
drop role if exists seller;
drop role if exists anonymous;
drop role if exists authenticator;
drop role if exists authenticated;

create database $name;
SQL

psql "$host" --set=ON_ERROR_STOP=true --file="packages/schema/src/lib.sql"

psql "$host" --set=ON_ERROR_STOP=true \
<<SQL
alter role authenticator with password 'password';
SQL
