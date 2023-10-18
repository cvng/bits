# https://www.postgresql.org/docs/current/app-psql.html

set -e
source .env

host="$DB_HOST"
name="$DB_NAME"
file="docs/seed.sql"

psql "$host" \
    --command="\connect $name" \
    --variable=ON_ERROR_STOP=1 \
    --file="$file"
