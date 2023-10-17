-- Domains

create domain amount as int;
create domain email as text check (value != '');
create domain id as uuid;

-- Roles

create role bidder;
create role reader;
create role seller;

-- Tables

create schema auth;
create schema live;
create schema shop;

-- -- Schema: auth

create table auth.person (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  username email not null
);

alter table auth.person enable row level security;

-- -- Schema: live

create table live.comment (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  author_id id not null references auth.person (id),
  show_id id not null references live.show (id),
  text text not null
);

alter table live.comment enable row level security;

create table live.show (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  creator_id id not null references auth.person (id),
  name text not null,
  started timestamp
);

alter table live.show enable row level security;

-- -- Schema: shop

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

create table shop.bid (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  auction_id id not null references shop.auction (id),
  bidder_id id not null references auth.person (id),
  amount amount not null
);

alter table shop.bid enable row level security;

create table shop.product (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  name text not null
);

alter table shop.product enable row level security;

-- Policies

create policy live_comment_create
on live.comment for insert to bidder
using (author_id = current_setting('auth.person_id')::id);

create policy live_comment_read
on live.comment for select to reader
using (true);

create policy live_show_create
on live.show for insert to seller
using (creator_id = current_setting('auth.person_id')::id);

create policy live_show_read
on live.show for select to reader
using (true);

create policy shop_bid_create
on shop.bid for insert to bidder
using (bidder_id = current_setting('auth.person_id')::id);

create policy shop_bid_read
on shop.bid for select to reader
using (true);

-- Functions

create or replace function check_bid_amount() returns trigger as $$
declare max_amount amount;
begin
	select max(amount) into max_amount
  from bid
  where bid.auction_id = new.auction_id;

	if new.amount <= max_amount then
		raise using errcode = 'lower_bid_amount';
	end if;

	return new;
end;
$$ language plpgsql;
