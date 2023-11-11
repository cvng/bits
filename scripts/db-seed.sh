#!/usr/bin/env bash
set -eu -o pipefail

# https://www.postgresql.org/docs/current/app-psql.html

source .env

env_host="$DATABASE_URL"
host="postgres://postgres:password@localhost:5432/bits"
name="bits"
file="scripts/data/events.json"

cargo task db-migrate > /dev/null

# Seed data from JSON event "stream" & test permissions

jq --compact-output ".[]" "$file" | psql "$host" --set=ON_ERROR_STOP=true \
    --command="create table tmp (row jsonb);" \
    --command="grant all on tmp to public;" \
    --command="\copy tmp (row) from stdin;"

psql "$host" --set=ON_ERROR_STOP=true \
<<SQL
insert into auth.person (id, email, role)
values ('00000000-0000-0000-0000-000000000000', 'admin@test.dev', 'admin');

insert into shop.config default values;
SQL

psql "$env_host" --set=ON_ERROR_STOP=true \
<<SQL
set plpgsql.print_strict_params to true;


do \$$ declare event jsonb; begin for event in select row from tmp loop
perform auth.login((event->>'user_id')::id);

insert into cqrs.event (user_id, type, data)
values (
    (event->>'user_id')::id,
    (event->>'type')::cqrs.event_type,
    (event->>'data')::jsonb
);
end loop; end; \$$;
SQL

psql "$host" --set=ON_ERROR_STOP=true \
<<SQL
select id, created, type, data->>'id' as data_id from cqrs.event;

drop table tmp;
SQL
