-- CQRS / ES https://github.com/cvng/bits/tree/main/docs/es.sql (work@cvng.dev)

create schema cqrs;

--
-- Enums
--

create type cqrs.event_type as enum (
  'auction_created',
  'bid_created',
  'comment_created',
  'person_created',
  'product_created',
  'show_created'
);

--
-- Types
--

create type cqrs.auction_created as (
  id id,
  show_id id,
  product_id id
);

create type cqrs.bid_created as (
  id id,
  auction_id id,
  bidder_id id,
  amount amount
);

create type cqrs.comment_created as (
  id id,
  author_id id,
  show_id id,
  text text
);

create type cqrs.person_created as (
  id id,
  email email
);

create type cqrs.product_created as (
  id id,
  name text
);

create type cqrs.show_created as (
  id id,
  creator_id id,
  name text
);

--
-- Tables
--

create table cqrs.event (
  id serial not null primary key,
  created timestamp not null default now(),
  type cqrs.event_type not null,
  data jsonb not null
);

alter table cqrs.event enable row level security;

--
-- Triggers
--

create function cqrs.event_insert_trigger() returns trigger
as $$
begin
  case new.type
    when 'auction_created' then
      perform cqrs.auction_created_handler(jsonb_populate_record(null::cqrs.auction_created, new.data));

    when 'bid_created' then
      perform cqrs.bid_created_handler(jsonb_populate_record(null::cqrs.bid_created, new.data));

    when 'comment_created' then
      perform cqrs.comment_created_handler(jsonb_populate_record(null::cqrs.comment_created, new.data));

    when 'person_created' then
      perform cqrs.person_created_handler(jsonb_populate_record(null::cqrs.person_created, new.data));

    when 'product_created' then
      perform cqrs.product_created_handler(jsonb_populate_record(null::cqrs.product_created, new.data));

    when 'show_created' then
      perform cqrs.show_created_handler(jsonb_populate_record(null::cqrs.show_created, new.data));
  end case;

  return new;
end;
$$ language plpgsql;

create trigger event_insert_trigger after insert on cqrs.event
for each row execute function cqrs.event_insert_trigger();

--
-- Functions
--

create function cqrs.auction_created_handler(event cqrs.auction_created) returns void -- noqa: LT05
as $$
begin
  insert into shop.auction (id, show_id, product_id)
  values (event.id, event.show_id, event.product_id);
end;
$$ language plpgsql;

create function cqrs.bid_created_handler(event cqrs.bid_created) returns void
as $$
begin
  insert into shop.bid (id, auction_id, bidder_id, amount)
  values (event.id, event.auction_id, event.bidder_id, event.amount);
end;
$$ language plpgsql;

create function cqrs.comment_created_handler(event cqrs.comment_created) returns void -- noqa: LT05
as $$
begin
  insert into live.comment (id, author_id, show_id, text)
  values (event.id, event.author_id, event.show_id, event.text);
end;
$$ language plpgsql;

create function cqrs.person_created_handler(event cqrs.person_created) returns void -- noqa: LT05
as $$
begin
  insert into auth.person (id, email)
  values (event.id, event.email);
end;
$$ language plpgsql;

create function cqrs.product_created_handler(event cqrs.product_created) returns void -- noqa: LT05
as $$
begin
  insert into shop.product (id, name)
  values (event.id, event.name);
end;
$$ language plpgsql;

create function cqrs.show_created_handler(event cqrs.show_created) returns void
as $$
begin
  insert into live.show (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end;
$$ language plpgsql;
