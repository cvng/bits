-- Handler: cqrs.auction_expired_handler

create function cqrs.auction_expired_handler(event cqrs.auction_expired) returns void
language plpgsql as $$
declare
  auction shop.auction;
begin
  update shop.auction
  set expired_at = event.expired_at
  where id = event.id
  returning * into strict auction;
end; $$;
