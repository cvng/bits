set -e

export $(cat .env | xargs)

SOURCE="$HASURA_GRAPHQL_DATABASE_URL"
TARGET="migrations/default/$(date +%s000)_todo"

psql "$SOURCE" \
    --command="drop database if exists shadow;" \
    --command="create database shadow;"

psql "$SOURCE/shadow" \
    --file="docs/schema.sql" \
    --single-transaction

mkdir -p "$TARGET"

migra \
    --unsafe \
    --exclude_schema="hdb_catalog" \
    "$SOURCE/postgres" "$SOURCE/shadow" \
    > "$TARGET/up.sql" || true

migra \
    --unsafe \
    --exclude_schema="hdb_catalog" \
    "$SOURCE/shadow" "$SOURCE/postgres" \
    > "$TARGET/down.sql" || true
