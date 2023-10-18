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

create domain amount as int;
create domain id as uuid;
create domain email as text check (value != '');

--
-- Tables
--

create schema auth;
create schema live;
create schema shop;

-- Table: auth.person

create table auth.person (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  username email not null
);

alter table auth.person enable row level security;

-- Table: live.show

create table live.show (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  creator_id id not null references auth.person (id),
  name text not null,
  started timestamp
);

alter table live.show enable row level security;

-- Table: live.comment

create table live.comment (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  author_id id not null references auth.person (id),
  show_id id not null references live.show (id),
  text text not null
);

alter table live.comment enable row level security;

-- Table: shop.product

create table shop.product (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  name text not null
);

alter table shop.product enable row level security;

-- Table: shop.auction

create table shop.auction (
  id id not null default gen_random_uuid() primary key,
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
  id id not null default gen_random_uuid() primary key,
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

create policy live_comment_create on live.comment for insert to bidder
with check (author_id = current_setting('auth.person_id')::id);

create policy live_comment_read on live.comment for select to viewer
using (true);

create policy live_show_create on live.show for insert to seller
with check (creator_id = current_setting('auth.person_id')::id);

create policy live_show_read on live.show for select to viewer
using (true);

create policy shop_bid_create on shop.bid for insert to bidder
with check (bidder_id = current_setting('auth.person_id')::id);

create policy shop_bid_read on shop.bid for select to viewer
using (true);

--
-- Functions
--

create function shop.set_bid_concurrent_amount() returns trigger as $$
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

--
-- Triggers
--

create trigger set_bid_concurrent_amount_trigger before insert on shop.bid
for each row execute function shop.set_bid_concurrent_amount();