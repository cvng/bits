#!/bin/zsh
set -o errexit -o nounset -o pipefail

# https://www.postgresql.org/docs/current/app-psql.html

source .env

host="$DATABASE_URL"
name="bits"

psql "$host" \
<<SQL
\connect postgres;

drop database if exists $name with (force);
drop role if exists admin;
drop role if exists bidder;
drop role if exists seller;
drop role if exists viewer;

create database $name;
SQL

psql "$host" --file="docs/schema.sql"
