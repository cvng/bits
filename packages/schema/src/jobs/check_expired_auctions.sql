-- Job: jobs.check_expired_auctions

create function jobs.check_expired_auctions() returns void
language plpgsql as $$
declare
  expired_auction shop.auction_session;
begin
  for expired_auction in
    select *
    from shop.auction_session
    where expires_at < clock_timestamp()
  loop
    insert into cqrs.event (user_id, type, data)
    values (
      auth.admin(),
      'auction_expired'::cqrs.event_type,
      json_build_object(
        'id', expired_auction.auction_id,
        'expired_at', expired_auction.expires_at
      )::jsonb
    );
  end loop;
end; $$;
