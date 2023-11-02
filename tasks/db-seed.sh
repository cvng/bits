#!/usr/bin/env bash
set -eu -o pipefail

# https://www.postgresql.org/docs/current/app-psql.html

source .env

# Seed data from JSON event "stream" & test permissions

host="$DATABASE_URL"
name="bits"
file="tasks/data/events.json"

psql "$host" --set=ON_ERROR_STOP=true --command="create table tmp (row jsonb);"

jq --compact-output ".[]" "$file" | psql "$host" --set=ON_ERROR_STOP=true \
    --command="\copy tmp (row) from stdin;"

psql "$host" --set=ON_ERROR_STOP=true \
<<SQL
insert into auth.person (id, email, role)
values ('00000000-0000-0000-0000-000000000000', 'admin@test.dev', 'admin');

do \$$
declare
    event jsonb;
begin
    for event in select row from tmp loop
        perform auth.login((event->>'user')::id);

        insert into cqrs.event (type, data)
        values ((event->>'type')::cqrs.event_type, (event->>'data')::jsonb);
    end loop;
end; \$$;

do \$$
begin
    perform auth.login('00000000-0000-0000-0000-000000000000');
end; \$$;
select id, created, type, data->>'id' as "data.id" from cqrs.event;
SQL

psql "$host" --set=ON_ERROR_STOP=true --command="drop table tmp;"
