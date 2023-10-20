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

create function cqrs.auction_created_trigger() returns trigger as $$
begin
  insert into shop.auction (id, show_id, product_id)
  values (new.id, new.show_id, new.product_id);

  return new;
end;
$$ language plpgsql;

create function cqrs.bid_created_trigger() returns trigger as $$
begin
  insert into shop.bid (id, auction_id, bidder_id, amount)
  values (new.id, new.auction_id, new.bidder_id, new.amount);

  return new;
end;
$$ language plpgsql;

create function cqrs.comment_created_trigger() returns trigger as $$
begin
  insert into live.comment (id, author_id, show_id, text)
  values (new.id, new.author_id, new.show_id, new.text);

  return new;
end;
$$ language plpgsql;

create function cqrs.person_created_trigger() returns trigger as $$
begin
  insert into auth.person (id, email)
  values (new.id, new.email);

  return new;
end;
$$ language plpgsql;

create function cqrs.product_created_trigger() returns trigger as $$
begin
  insert into shop.product (id, name)
  values (new.id, new.name);

  return new;
end;
$$ language plpgsql;

create function cqrs.show_created_trigger() returns trigger as $$
begin
  insert into live.show (id, creator_id, name)
  values (new.id, new.creator_id, new.name);

  return new;
end;
$$ language plpgsql;

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
