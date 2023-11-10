-- Handler: cqrs.event_insert_handler

create function cqrs.event_insert_handler(event cqrs.event) returns void
language plpgsql as $$
begin
  perform auth.login(event.user_id);

  case event.type
    when 'auction_created' then
      perform cqrs.auction_created_handler(
        jsonb_populate_record(null::cqrs.auction_created, event.data));

   when 'auction_started' then
      perform cqrs.auction_started_handler(
        jsonb_populate_record(null::cqrs.auction_started, event.data));

    when 'bid_created' then
      perform cqrs.bid_created_handler(
        jsonb_populate_record(null::cqrs.bid_created, event.data));

    when 'comment_created' then
      perform cqrs.comment_created_handler(
        jsonb_populate_record(null::cqrs.comment_created, event.data));

    when 'person_created' then
      perform cqrs.person_created_handler(
        jsonb_populate_record(null::cqrs.person_created, event.data));

    when 'product_created' then
      perform cqrs.product_created_handler(
        jsonb_populate_record(null::cqrs.product_created, event.data));

    when 'show_created' then
      perform cqrs.show_created_handler(
        jsonb_populate_record(null::cqrs.show_created, event.data));

    when 'show_started' then
      perform cqrs.show_started_handler(
        jsonb_populate_record(null::cqrs.show_started, event.data));
  end case;

  perform cqrs.notify(event);
end; $$;

-- Handler: cqrs.auction_created_handler

create function cqrs.auction_created_handler(event cqrs.auction_created) returns void
language plpgsql as $$
declare
  config shop.config;
begin
  select * from shop.config() into config;

  insert into shop.auction (id, show_id, product_id)
  values (
    event.id,
    event.show_id,
    event.product_id
  );
end; $$;

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
    concurrent_amount
  )
  values (
    event.id,
    event.auction_id,
    event.bidder_id,
    event.amount,
    session.max_amount
  )
  returning * into strict bid;

  update shop.auction_session set
    max_amount = bid.amount,
    expires_at = session.expires_at + session.refresh_secs
  where id = session.id
  returning id into strict session;
end; $$;

-- Handler: cqrs.comment_created_handler

create function cqrs.comment_created_handler(event cqrs.comment_created) returns void
language plpgsql as $$
begin
  insert into live.comment (id, author_id, show_id, text)
  values (event.id, event.author_id, event.show_id, event.text);
end; $$;

-- Handler: cqrs.person_created_handler

create function cqrs.person_created_handler(event cqrs.person_created) returns void
language plpgsql as $$
begin
  insert into auth.person (id, email, role)
  values (event.id, event.email, event.role);
end; $$;

-- Handler: cqrs.product_created_handler

create function cqrs.product_created_handler(event cqrs.product_created) returns void
language plpgsql as $$
begin
  insert into shop.product (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end; $$;

-- Handler: cqrs.show_created_handler

create function cqrs.show_created_handler(event cqrs.show_created) returns void
language plpgsql as $$
begin
  insert into live.show (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end; $$;

-- Handler: cqrs.show_started_handler

create function cqrs.show_started_handler(event cqrs.show_started) returns void
language plpgsql as $$
declare
  config shop.config;
  show live.show;
begin
  update live.show
  set
    started_at = clock_timestamp(),
    started = not started
  where id = event.id
  returning id into strict show;
end; $$;
