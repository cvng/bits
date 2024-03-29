#!/usr/bin/env bash
set -eu -o pipefail

source .env

env_host="$DATABASE_URL"
host="postgres://postgres:password@localhost:5432/bits"

cargo task db-migrate > /dev/null
cargo task db-seed > /dev/null

psql "$host" --set=ON_ERROR_STOP=true \
<<SQL
set plpgsql.print_strict_params to true;

update shop.auction_session set expires_at = clock_timestamp() + '-1'::interval
where auction_id = (
    select id from shop.auction
    where id = '00000000-0000-0000-0000-000000000000'
);
SQL

psql "$env_host" --set=ON_ERROR_STOP=true \
<<SQL
do \$$ begin
perform auth.login('00000000-2000-0000-0000-000000000000'::id);

insert into cqrs.event (user_id, type, data)
values (
    '00000000-2000-0000-0000-000000000000',
    'bid_created',
    '{
        "id": "c861f1ca-6105-4767-999f-dd70a98249f0",
        "auction_id": "00000000-0000-0000-0000-000000000000",
        "buyer_id": "00000000-2000-0000-0000-000000000000",
        "amount": 500}'
);
end; \$$;
SQL
