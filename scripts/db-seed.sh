#!/usr/bin/env bash
set -eu -o pipefail

# https://www.postgresql.org/docs/current/app-psql.html

source .env

host="$DATABASE_URL"
name="bits"
file="scripts/data/events.json"

cargo task db-migrate > /dev/null

# Seed data from JSON event "stream" & test permissions

jq --compact-output ".[]" "$file" | psql "$host" --set=ON_ERROR_STOP=true \
    --command="create table tmp (row jsonb);" \
    --command="\copy tmp (row) from stdin;"

psql "$host" --set=ON_ERROR_STOP=true \
<<SQL
set plpgsql.print_strict_params to true;

insert into auth.person (id, email, role)
values ('00000000-0000-0000-0000-000000000000', 'admin@test.dev', 'admin');

insert into shop.config default values;

insert into cqrs.event (user_id, type, data)
select
    (row->>'user_id')::id,
    (row->>'type')::cqrs.event_type,
    (row->>'data')::jsonb
from tmp;

select id, created, type, data->>'id' as data_id from cqrs.event;
SQL

psql "$host" --set=ON_ERROR_STOP=true --command="drop table tmp;"
