# https://www.postgresql.org/docs/current/app-psql.html

set -e
source .env

host="$DATABASE_URL"

# Seed

jq --compact-output ".[]" tasks/seed.json | psql "postgres://postgres:password@localhost:5432/bits" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --command="\connect bits;" \
    --command="create table tmp (row jsonb);" \
    --command="grant select on tmp to public;" \
    --command="\copy tmp (row) from stdin;" \
    --quiet

psql "$host" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --command="\connect bits;" \
    --command="set role viewer;" \
    --command="do \$$ begin perform set_config('auth.user_id', '00000000-0000-0000-0000-000000000000', false); end \$$;" \
    --command="insert into cqrs.event (type, data) select (row->>'type')::cqrs.event_type, (row->>'data')::jsonb from tmp;" \
