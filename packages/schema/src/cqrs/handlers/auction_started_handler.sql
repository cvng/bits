-- Handler: cqrs.auction_started_handler

create function cqrs.auction_started_handler(event cqrs.auction_started) returns void
language plpgsql as $$
declare
  now timestamptz;
  config shop.config;
  auction shop.auction;
  session shop.auction_session;
begin
  select clock_timestamp() into now;
  select * from shop.config() into config;

  insert into shop.auction_session (
    id,
    auction_id,
    max_amount,
    timeout_secs,
    refresh_secs,
    expires_at
  )
  values (
    gen_random_uuid(),
    event.id,
    default,
    config.auction_timeout_secs,
    config.auction_refresh_secs,
    config.auction_timeout_secs + now
  )
   returning * into strict session;

  update shop.auction set started_at = session.created
  where id = event.id returning id into strict auction;
end; $$;
