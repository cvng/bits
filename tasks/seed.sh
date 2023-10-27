# https://www.postgresql.org/docs/current/app-psql.html

set -e
source .env

# Seed data from JSON event "stream" & test permissions

host="postgres://postgres:password@localhost:5432/bits"
name="bits"
file="tasks/seed.json"

psql "$host" --no-psqlrc --variable=ON_ERROR_STOP=1 --quiet \
<<SQL
\connect $name;

create table if not exists tmp (row jsonb);
grant select on tmp to public; -- TODO
SQL

jq --compact-output ".[]" "$file" | psql "$host" --no-psqlrc \
    --variable=ON_ERROR_STOP=1 --quiet --command="\copy tmp (row) from stdin;"

psql "$DATABASE_URL" --no-psqlrc --variable=ON_ERROR_STOP=1 --quiet \
<<SQL
\connect $name;

select auth.register('00000000-0000-0000-0000-000000000000', 'admin', 'admin@test.dev');
select auth.register('00000000-1000-0000-0000-000000000000', 'seller', 'seller@test.dev');
select auth.register('00000000-2000-0000-0000-000000000000', 'bidder', 'bidder@test.dev');

select auth.login('00000000-1000-0000-0000-000000000000');
insert into cqrs.event (type, data)
select (row->>'type')::cqrs.event_type, (row->>'data')::jsonb
from tmp where row->>'user' = '00000000-1000-0000-0000-000000000000';

select auth.login('00000000-2000-0000-0000-000000000000');
insert into cqrs.event (type, data)
select (row->>'type')::cqrs.event_type, (row->>'data')::jsonb
from tmp where row->>'user' = '00000000-2000-0000-0000-000000000000';

select auth.login('00000000-0000-0000-0000-000000000000');
select id, created, type, data->>'id' as "data.id" from cqrs.event;
SQL
