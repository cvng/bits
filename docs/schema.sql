-- https://github.com/cvng/bits/tree/main/docs/schema.sql (work@cvng.dev)

--
-- Schema
--

create schema auth;
create schema cqrs;
create schema live;
create schema shop;

--
-- Roles
--

create role admin;
create role bidder;
create role seller;
create role viewer noinherit;

grant viewer to bidder;
grant bidder to seller;
grant seller to admin;

grant usage on schema auth to viewer;
grant usage on schema cqrs to viewer;
grant usage on schema live to bidder;
grant usage on schema shop to bidder;

--
-- Domains
--

create domain id as uuid;
create domain amount as numeric;
create domain email as text check (value = lower(value) and value like '%@%');

--
-- Enums
--

create type auth.role as enum (
  'admin',
  'bidder',
  'seller',
  'viewer'
);

create type cqrs.event_type as enum (
  'auction_created',
  'bid_created',
  'comment_created',
  'person_created',
  'product_created',
  'show_created',
  'show_started'
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
  email email,
  role auth.role
);

create type cqrs.product_created as (
  id id,
  creator_id id,
  name text
);

create type cqrs.show_created as (
  id id,
  creator_id id,
  name text
);

create type cqrs.show_started as (
  id id
);

--
-- Tables
--

-- Table: auth.person

create table auth.person (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  email email not null unique,
  role auth.role not null default 'viewer'::auth.role
);

alter table auth.person enable row level security;

-- Table: cqrs.event

create table cqrs.event (
  id bigint not null primary key generated always as identity,
  created timestamptz not null default clock_timestamp(),
  user_id id not null references auth.person (id),
  type cqrs.event_type not null,
  data jsonb not null
);

alter table cqrs.event enable row level security;

-- Table: live.show

create table live.show (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  creator_id id not null references auth.person (id),
  name text not null,
  started_at timestamptz,
  started boolean default false
);

alter table live.show enable row level security;

-- Table: live.comment

create table live.comment (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  author_id id not null references auth.person (id),
  show_id id not null references live.show (id),
  text text not null
);

alter table live.comment enable row level security;

-- Table: shop.product

create table shop.product (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  creator_id id not null references auth.person (id),
  name text not null
);

alter table shop.product enable row level security;

-- Table: shop.auction

create table shop.auction (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  show_id id not null references live.show (id),
  product_id id not null references shop.product (id),
  started_at timestamptz,
  expired_at timestamptz,
  timeout_secs interval not null,
  refresh_secs interval not null
);

alter table shop.auction enable row level security;

-- Table: shop.bid

create table shop.bid (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  auction_id id not null references shop.auction (id),
  bidder_id id not null references auth.person (id),
  concurrent_amount amount not null default 0,
  amount amount not null
);

alter table shop.bid enable row level security;

-- Table: shop.config

create table shop.config (
  id id not null primary key,
  show_id id not null references live.show (id),
  auction_timeout_secs interval not null default '60',
  auction_refresh_secs interval not null default '15'
);

alter table shop.config enable row level security;

--
-- Checks
--

-- Check: show_started_check

alter table live.show add constraint show_started_check
check (
  (started_at is null and not started) or
  (started_at is not null and started)
);

-- Check: bid_amount_check

alter table shop.bid add constraint bid_amount_check
check (amount > concurrent_amount);

--
-- Privileges
--

-- Table: cqrs.event

grant select on cqrs.event to viewer;
grant insert on cqrs.event to viewer;

-- Table: auth.person

grant select on auth.person to viewer;
grant insert on auth.person to admin;

-- Table: live.comment

grant select on live.comment to viewer;
grant insert on live.comment to bidder;

-- Table: shop.bid

grant select on shop.bid to viewer;
grant insert on shop.bid to bidder;

-- Table: live.show

grant select on live.show to viewer;
grant insert on live.show to seller;
grant update on live.show to seller;

-- Table: shop.auction

grant select on shop.auction to viewer;
grant insert on shop.auction to seller;
grant update on shop.auction to seller;

-- Table: shop.config

grant select on shop.config to seller;
grant insert on shop.config to seller;

-- Table: shop.product

grant select on shop.product to viewer;
grant insert on shop.product to seller;

--
-- Utilities
--

create function auth.login(user_id id) returns void as $$
declare
  enabled_role auth.role;
begin
  select role into strict enabled_role from auth.person where id = user_id;

  perform set_config('role', enabled_role::text, true);
  perform set_config('auth.user', user_id::text, true);
end;
$$ language plpgsql;

create function auth.role() returns auth.role as $$
begin
  return (current_setting('role'))::auth.role;
end;
$$ language plpgsql;

create function auth.user() returns id as $$
begin
  return (current_setting('auth.user'))::id;
end;
$$ language plpgsql;

--
-- Policies
--

-- Table: cqrs.event

create policy event_select_policy on cqrs.event for select to viewer
using ('admin'::auth.role = auth.role());

create policy event_insert_policy on cqrs.event for insert to viewer
with check (true);

-- Table: auth.person

create policy person_select_policy on auth.person for select to viewer
using (true);

create policy person_insert_policy on auth.person for insert to admin
with check (true);

-- Table: live.show

create policy show_select_policy on live.show for select to viewer
using (true);

create policy show_insert_policy on live.show for insert to seller
with check (creator_id = auth.user());

create policy show_update_policy on live.show for update to seller
using (true) with check (creator_id = auth.user());

-- Table: live.comment

create policy comment_select_policy on live.comment for select to viewer
using (true);

create policy comment_insert_policy on live.comment for insert to bidder
with check (author_id = auth.user());

-- Table: shop.product

create policy product_select_policy on shop.product for select to viewer
using (true);

create policy product_insert_policy on shop.product for insert to seller
with check (creator_id = auth.user());

-- Table: shop.auction

create policy auction_select_policy on shop.auction for select to viewer
using (true);

create policy auction_insert_policy on shop.auction for insert to seller
with check (
  show_id in (select id from live.show where creator_id = auth.user()) and
  product_id in (select id from shop.product where creator_id = auth.user())
);

create policy auction_update_policy on shop.auction for update to seller
using (true) with check (
  show_id in (select id from live.show where creator_id = auth.user()) and
  product_id in (select id from shop.product where creator_id = auth.user())
);

-- Table: shop.bid

create policy bid_select_policy on shop.bid for select to viewer
using (true);

create policy bid_insert_policy on shop.bid for insert to bidder
with check (bidder_id = auth.user());

-- Table: shop.config

create policy config_select_policy on shop.config for select to seller
using (true);

create policy config_insert_policy on shop.config for insert to seller
with check (
  show_id in (select id from live.show where creator_id = auth.user())
);

--
-- Triggers
--

create function cqrs.event_insert_trigger() returns trigger as $$
begin
  perform auth.login(new.user_id);

  case new.type
    when 'auction_created' then
      perform cqrs.auction_created_handler(
        jsonb_populate_record(null::cqrs.auction_created, new.data));

    when 'bid_created' then
      perform cqrs.bid_created_handler(
        jsonb_populate_record(null::cqrs.bid_created, new.data));

    when 'comment_created' then
      perform cqrs.comment_created_handler(
        jsonb_populate_record(null::cqrs.comment_created, new.data));

    when 'person_created' then
      perform cqrs.person_created_handler(
        jsonb_populate_record(null::cqrs.person_created, new.data));

    when 'product_created' then
      perform cqrs.product_created_handler(
        jsonb_populate_record(null::cqrs.product_created, new.data));

    when 'show_created' then
      perform cqrs.show_created_handler(
        jsonb_populate_record(null::cqrs.show_created, new.data));

    when 'show_started' then
      perform cqrs.show_started_handler(
        jsonb_populate_record(null::cqrs.show_started, new.data));
  end case;

  perform cqrs.handler(new);

  return new;
end;
$$ language plpgsql;

create trigger event_insert_trigger after insert on cqrs.event
for each row execute function cqrs.event_insert_trigger();

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
  select * into strict config
  from shop.config where show_id = event.show_id;

  insert into shop.auction (id, show_id, product_id, timeout_secs, refresh_secs)
  values (event.id, event.show_id, event.product_id, config.auction_timeout_secs, config.auction_refresh_secs);
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

  insert into shop.config (id, show_id)
  values (event.id, event.id); -- TODO: re-use same id?
end;
$$ language plpgsql;

create function cqrs.show_started_handler(event cqrs.show_started)
returns void as $$
declare
  v_show_id id;
  auction_timeout_secs interval;
  auction_refresh_secs interval;
begin
  select auction.show_id into strict v_show_id from shop.auction where id = event.id;
  select config.auction_timeout_secs into strict auction_timeout_secs from shop.config where config.show_id = v_show_id; -- TODO: single query
  select config.auction_refresh_secs into strict auction_refresh_secs from shop.config where config.show_id = v_show_id; -- TODO: single query

  update live.show set started_at = clock_timestamp(), started = not started
  where id = v_show_id;

  update shop.auction
  set
    started_at = clock_timestamp(),
    expired_at = clock_timestamp() + auction_timeout_secs
  where id = event.id;
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
    started_at,
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
    started_at,
    expired_at
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
