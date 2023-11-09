--
-- Handlers
--

create function cqrs.handler(event cqrs.event) returns void as $$
begin
  perform pg_notify(
    'cqrs.event',
    jsonb_build_object(
      'id', event.id,
      'created', event.created,
      'user_id', event.user_id,
      'type', event.type,
      'data', event.data
    )::text
  );
end;
$$ language plpgsql;

create function cqrs.auction_created_handler(event cqrs.auction_created)
returns void as $$
declare
  config shop.config;
begin
  select * into strict config from shop.config;

  insert into shop.auction (id, show_id, product_id, timeout_secs, refresh_secs)
  values (
    event.id,
    event.show_id,
    event.product_id,
    config.auction_timeout_secs,
    config.auction_refresh_secs
  );
end;
$$ language plpgsql;

create function cqrs.bid_created_handler(event cqrs.bid_created)
returns void as $$
declare
  current_max_amount amount;
begin
  select max(amount) into strict current_max_amount
  from shop.bid where auction_id = event.auction_id;

  insert into shop.bid (id, auction_id, bidder_id, concurrent_amount, amount)
  values (
    event.id,
    event.auction_id,
    event.bidder_id,
    coalesce(current_max_amount, 0),
    event.amount
  );
end;
$$ language plpgsql;

create function cqrs.comment_created_handler(event cqrs.comment_created)
returns void as $$
begin
  insert into live.comment (id, author_id, show_id, text)
  values (event.id, event.author_id, event.show_id, event.text);
end;
$$ language plpgsql;

create function cqrs.person_created_handler(event cqrs.person_created)
returns void as $$
begin
  insert into auth.person (id, email, role)
  values (event.id, event.email, event.role);
end;
$$ language plpgsql;

create function cqrs.product_created_handler(event cqrs.product_created)
returns void as $$
begin
  insert into shop.product (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end;
$$ language plpgsql;

create function cqrs.show_created_handler(event cqrs.show_created)
returns void as $$
begin
  insert into live.show (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end;
$$ language plpgsql;

create function cqrs.show_started_handler(event cqrs.show_started)
returns void as $$
declare
  auction shop.auction;
  config shop.config;
  show live.show;
begin
  select * into strict config from shop.config;

  update live.show
  set
    started_at = clock_timestamp(),
    started = not started
  where id = (select show_id from shop.auction where id = event.id)
  returning id into strict show;

  update shop.auction
  set
    started_at = clock_timestamp(),
    expired_at = clock_timestamp() + config.auction_timeout_secs
  where id = event.id
  returning id into strict auction;
end;
$$ language plpgsql;
