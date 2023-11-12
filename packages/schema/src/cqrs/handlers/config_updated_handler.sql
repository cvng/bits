-- Handler: cqrs.config_updated_handler

create function cqrs.config_updated_handler(event cqrs.config_updated) returns void
language plpgsql as $$
declare
  config shop.config;
begin
  update shop.config
  set
    auction_timeout_secs = coalesce(event.auction_timeout_secs, auction_timeout_secs),
    auction_refresh_secs = coalesce(event.auction_refresh_secs, auction_refresh_secs)
  returning * into strict config;
end; $$;
