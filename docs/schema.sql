-- Schema https://github.com/cvng/bits/tree/main/docs/schema.sql (work@cvng.dev)

create schema auth;
create schema cqrs;
create schema live;
create schema shop;

--
-- Roles
--

drop role if exists bidder;
drop role if exists seller;
drop role if exists viewer;

create role bidder;
create role seller;
create role viewer;

--
-- Domains
--

create domain id as uuid;
create domain amount as numeric;
create domain email as text check (value = lower(value) and value like '%@%');

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

-- Table: cqrs.event

create table cqrs.event (
  id serial not null primary key,
  created timestamp not null default clock_timestamp(),
  type cqrs.event_type not null,
  data jsonb not null
);

alter table cqrs.event enable row level security;

-- Table: auth.person

create table auth.person (
  id id not null primary key,
  created timestamp not null default clock_timestamp(),
  updated timestamp,
  email email not null
);

alter table auth.person enable row level security;

-- Table: live.show

create table live.show (
  id id not null primary key,
  created timestamp not null default clock_timestamp(),
  updated timestamp,
  creator_id id not null references auth.person (id),
  name text not null,
  started timestamp
);

alter table live.show enable row level security;

-- Table: live.comment

create table live.comment (
  id id not null primary key,
  created timestamp not null default clock_timestamp(),
  updated timestamp,
  author_id id not null references auth.person (id),
  show_id id not null references live.show (id),
  text text not null
);

alter table live.comment enable row level security;

-- Table: shop.product

create table shop.product (
  id id not null primary key,
  created timestamp not null default clock_timestamp(),
  updated timestamp,
  name text not null
);

alter table shop.product enable row level security;

-- Table: shop.auction

create table shop.auction (
  id id not null primary key,
  created timestamp not null default clock_timestamp(),
  updated timestamp,
  show_id id not null references live.show (id),
  product_id id not null references shop.product (id),
  started timestamp,
  expired timestamp
);

alter table shop.auction enable row level security;

-- Table: shop.bid

create table shop.bid (
  id id not null primary key,
  created timestamp not null default clock_timestamp(),
  updated timestamp,
  auction_id id not null references shop.auction (id),
  bidder_id id not null references auth.person (id),
  concurrent_amount amount not null default 0,
  amount amount not null check (amount > concurrent_amount)
);

alter table shop.bid enable row level security;

--
-- Policies
--

create policy comment_create_policy on live.comment for insert to bidder
with check (author_id = current_setting('auth.person_id')::id);

create policy comment_read_policy on live.comment for select to viewer
using (true);

create policy show_create_policy on live.show for insert to seller
with check (creator_id = current_setting('auth.person_id')::id);

create policy show_read_policy on live.show for select to viewer
using (true);

create policy bid_create_policy on shop.bid for insert to bidder
with check (bidder_id = current_setting('auth.person_id')::id);

create policy bid_read_policy on shop.bid for select to viewer
using (true);

create policy person_policy on auth.person
using (id = current_setting('auth.person_id')::id);

--
-- Triggers
--

create function cqrs.event_insert_trigger() returns trigger as $$
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
-- Handlers
--

create function cqrs.auction_created_handler(event cqrs.auction_created)
returns void as $$
begin
  insert into shop.auction (id, show_id, product_id)
  values (event.id, event.show_id, event.product_id);
end;
$$ language plpgsql;

create function cqrs.bid_created_handler(event cqrs.bid_created)
returns void as $$
declare
  current_max_amount amount;
begin
  select max(amount) into current_max_amount
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
  insert into auth.person (id, email)
  values (event.id, event.email);
end;
$$ language plpgsql;

create function cqrs.product_created_handler(event cqrs.product_created)
returns void as $$
begin
  insert into shop.product (id, name)
  values (event.id, event.name);
end;
$$ language plpgsql;

create function cqrs.show_created_handler(event cqrs.show_created)
returns void as $$
begin
  insert into live.show (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end;
$$ language plpgsql;

--
-- Views
--

create view public.person with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    email
  from auth.person
);

create view public.show with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    creator_id,
    name,
    started
  from live.show
);

create view public.comment with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    author_id,
    show_id,
    text
  from live.comment
);

create view public.product with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    name
  from shop.product
);

create view public.auction with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    show_id,
    product_id,
    started,
    expired
  from shop.auction
);

create view public.bid with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    auction_id,
    bidder_id,
    concurrent_amount,
    amount
  from shop.bid
);
