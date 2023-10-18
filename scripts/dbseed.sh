# https://www.postgresql.org/docs/current/app-psql.html

set -e
source .env

host="$DB_HOST"
name="$DB_NAME"

time psql "$host" \
    --command="\connect $name" \
    --file="docs/seed.sql" \
    --set ON_ERROR_STOP=1
