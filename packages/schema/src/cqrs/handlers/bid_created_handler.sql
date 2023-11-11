-- Handler: cqrs.bid_created_handler

create function cqrs.bid_created_handler(event cqrs.bid_created) returns void
language plpgsql as $$
declare
  bid shop.bid;
  session shop.auction_session;
begin
  select * into strict session
  from shop.auction_session where auction_id = event.auction_id;

  insert into shop.bid (
    id,
    auction_id,
    bidder_id,
    amount,
    concurrent_amount,
    session_expires_at
  )
  values (
    event.id,
    event.auction_id,
    event.bidder_id,
    event.amount,
    session.max_amount,
    session.expires_at
  )
  returning * into strict bid;

  update shop.auction_session
  set
    max_amount = bid.amount,
    expires_at = session.expires_at + session.refresh_secs
  where id = session.id
  returning id into strict session;
end; $$;
