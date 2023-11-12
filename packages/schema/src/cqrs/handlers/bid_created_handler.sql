-- Handler: cqrs.bid_created_handler

create function cqrs.bid_created_handler(event cqrs.bid_created) returns void
language plpgsql as $$
declare
  bid shop.bid;
  session shop.auction_session;
  new_expires_at timestamptz;
begin
  select * into strict session
  from shop.auction_session where auction_id = event.auction_id;

  insert into shop.bid (
    id,
    auction_id,
    buyer_id,
    amount,
    concurrent_amount,
    auction_expires_at
  )
  values (
    event.id,
    event.auction_id,
    event.buyer_id,
    event.amount,
    session.max_amount,
    session.expires_at
  )
  returning * into strict bid;

  if session.expires_at - bid.created < session.refresh_secs then
    new_expires_at := session.expires_at + session.refresh_secs;
  else
    new_expires_at := session.expires_at;
  end if;

  update shop.auction_session
  set
    max_amount = bid.amount,
    expires_at = new_expires_at
  where id = session.id
  returning * into strict session;
end; $$;
