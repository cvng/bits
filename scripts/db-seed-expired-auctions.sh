#!/usr/bin/env bash
set -eu -o pipefail

source .env

host="$DATABASE_URL"

cargo task db-seed > /dev/null

psql "$host" --set=ON_ERROR_STOP=true \
<<SQL
set plpgsql.print_strict_params to true;

update shop.auction_session set expires_at = clock_timestamp() + '-1'::interval
where auction_id = (
    select id from shop.auction
    where id = '00000000-0000-0000-0000-000000000000'
);

do \$$ begin
perform auth.login('00000000-0000-0000-0000-000000000000'::id);

perform jobs.check_expired_auctions();
end; \$$;
SQL
