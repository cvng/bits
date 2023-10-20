-- CQRS / ES https://github.com/cvng/bits/tree/main/docs/es.sql (work@cvng.dev)

create schema cqrs;

--
-- Enums
--

create type cqrs.event as enum (
  'auction_created',
  'bid_created',
  'comment_created',
  'person_created',
  'product_created',
  'show_created'
);

--
-- Tables
--

create table cqrs.auction_created (
  id id not null,
  show_id id not null,
  product_id id not null
);

create table cqrs.bid_created (
  id id not null,
  auction_id id not null,
  bidder_id id not null,
  amount amount not null
);

create table cqrs.comment_created (
  id id not null,
  author_id id not null,
  show_id id not null,
  text text not null
);

create table cqrs.person_created (
  id id not null,
  email email not null
);

create table cqrs.product_created (
  id id not null,
  name text not null
);

create table cqrs.show_created (
  id id not null,
  creator_id id not null,
  name text not null
);

--
-- Triggers
--

-- noqa: disable=LT05

create function cqrs.auction_created_trigger(event cqrs.auction_created) returns trigger as $$
begin
  insert into shop.auction (id, show_id, product_id)
  values (event.id, event.show_id, event.product_id);

  return new;
end;
$$ language plpgsql;

create function cqrs.bid_created_trigger(event cqrs.bid_created) returns trigger as $$
begin
  insert into shop.bid (id, auction_id, bidder_id, amount)
  values (event.id, event.auction_id, event.bidder_id, event.amount);

  return new;
end;
$$ language plpgsql;

create function cqrs.comment_created_trigger(event cqrs.comment_created) returns trigger as $$
begin
  insert into live.comment (id, author_id, show_id, text)
  values (event.id, event.author_id, event.show_id, event.text);

  return new;
end;
$$ language plpgsql;

create function cqrs.person_created_trigger(event cqrs.person_created) returns trigger as $$
begin
  insert into auth.person (id, email)
  values (event.id, event.email);

  return new;
end;
$$ language plpgsql;

create function cqrs.product_created_trigger(event cqrs.product_created) returns trigger as $$
begin
  insert into shop.product (id, name)
  values (event.id, event.name);

  return new;
end;
$$ language plpgsql;

create function cqrs.show_created_trigger(event cqrs.show_created) returns trigger as $$
begin
  insert into live.show (id, creator_id, name)
  values (event.id, event.creator_id, event.name);

  return new;
end;
$$ language plpgsql;

-- noqa: enable=LT05

create trigger auction_created_trigger after insert on cqrs.auction_created
for each row execute function cqrs.auction_created_trigger();

create trigger bid_created_trigger after insert on cqrs.bid_created
for each row execute function cqrs.bid_created_trigger();

create trigger comment_created_trigger after insert on cqrs.comment_created
for each row execute function cqrs.comment_created_trigger();

create trigger person_created_trigger after insert on cqrs.person_created
for each row execute function cqrs.person_created_trigger();

create trigger product_created_trigger after insert on cqrs.product_created
for each row execute function cqrs.product_created_trigger();

create trigger show_created_trigger after insert on cqrs.show_created
for each row execute function cqrs.show_created_trigger();
