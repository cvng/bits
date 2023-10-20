-- Schema https://github.com/cvng/bits/tree/main/docs/schema.sql (work@cvng.dev)

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
-- Tables
--

create schema auth;
create schema live;
create schema shop;

-- Table: auth.person

create table auth.person (
  id id not null primary key,
  created timestamp not null default now(),
  updated timestamp,
  email email not null
);

alter table auth.person enable row level security;

-- Table: live.show

create table live.show (
  id id not null primary key,
  created timestamp not null default now(),
  updated timestamp,
  creator_id id not null references auth.person (id),
  name text not null,
  started timestamp
);

alter table live.show enable row level security;

-- Table: live.comment

create table live.comment (
  id id not null primary key,
  created timestamp not null default now(),
  updated timestamp,
  author_id id not null references auth.person (id),
  show_id id not null references live.show (id),
  text text not null
);

alter table live.comment enable row level security;

-- Table: shop.product

create table shop.product (
  id id not null primary key,
  created timestamp not null default now(),
  updated timestamp,
  name text not null
);

alter table shop.product enable row level security;

-- Table: shop.auction

create table shop.auction (
  id id not null primary key,
  created timestamp not null default now(),
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
  created timestamp not null default now(),
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

create function shop.bid_insert_trigger() returns trigger
as $$
declare
  current_max_amount amount;
begin
  select max(amount) into current_max_amount
  from shop.bid where auction_id = new.auction_id;

  if current_max_amount is not null then
    new.concurrent_amount = current_max_amount;
  end if;

  return new;
end;
$$ language plpgsql;

create trigger bid_insert_trigger before insert on shop.bid
for each row execute function shop.bid_insert_trigger();

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
