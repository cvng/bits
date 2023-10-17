-- Domains

create domain amount as int;
create domain email as text check (value != '');
create domain id as uuid;

-- Roles

create role reader noinherit;

create role bidder;
create role seller;

-- Tables

-- -- Schema: auth

create table auth.person (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  username email not null
);

alter table auth.person enable row level security;

-- -- Schema: live

create table live.auction (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  show_id id not null references show.show (id),
  product_id id not null references shop.product (id),
  started timestamp,
  expired timestamp,
  winning_bid_id id references live.bid (id)
);

alter table live.auction enable row level security;

create table live.bid (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  auction_id id not null references live.auction (id),
  bidder_id id not null references auth.person (id),
  amount amount not null
);

alter table live.bid enable row level security;

-- -- Schema: shop

create table shop.product (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  name text not null
);

alter table shop.product enable row level security;

-- -- Schema: show

create table show.comment (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  author_id id not null references auth.person (id),
  show_id id not null references show.show (id),
  text text not null
);

alter table show.comment enable row level security;

create table show.show (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  creator_id id not null references auth.person (id),
  name text not null,
  started timestamp
);

alter table show.show enable row level security;

-- Policies

create policy create_bid
on live.bid for insert to bidder
using (bidder_id = current_setting('auth.person_id')::id);

create policy read_bid
on live.bid for select to reader
using (true);

create policy create_comment
on show.comment for insert to bidder
using (author_id = current_setting('auth.person_id')::id);

create policy read_comment
on show.comment for select to reader
using (true);

create policy create_show
on show.show for insert to seller
using (creator_id = current_setting('auth.person_id')::id);

create policy read_show
on show.show for select to reader
using (true);
