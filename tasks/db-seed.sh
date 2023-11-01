#!/bin/zsh
set -o errexit -o nounset -o pipefail

# https://www.postgresql.org/docs/current/app-psql.html

source .env

# Seed data from JSON event "stream" & test permissions

host="$DATABASE_URL"
name="bits"
file="tasks/data/events.json"

psql "$host" \
<<SQL
\connect $name;

create table if not exists tmp (row jsonb);
grant select on tmp to public;
SQL

jq --compact-output ".[]" "$file" | psql "$host" \
    --command="\copy tmp (row) from stdin;"

psql "$host" \
<<SQL
\connect $name;

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

do \$$ begin perform auth.login('00000000-0000-0000-0000-000000000000'); end; \$$;
select id, created, type, data->>'id' as "data.id" from cqrs.event;
SQL

psql "$host" --command="drop table tmp;"
