-- Domains

create domain amount as int;
create domain email as text check (value != '');
create domain id as uuid;

-- Roles

drop role if exists bidder;
drop role if exists reader;
drop role if exists seller;

create role bidder;
create role reader;
create role seller;

-- Tables

create schema auth;
create schema conf;
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

create table live.show (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  creator_id id not null references auth.person (id),
  name text not null,
  started timestamp
);

alter table live.show enable row level security;

create table live.comment (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  author_id id not null references auth.person (id),
  show_id id not null references live.show (id),
  text text not null
);

alter table live.comment enable row level security;

-- -- Schema: shop

create table shop.product (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  updated timestamp,
  name text not null
);

alter table shop.product enable row level security;

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

-- -- Schema: conf

create table conf.error (
  code char(5) not null primary key,
  message text not null unique,
  detail text,
  hint text
);

insert into conf.error (code, message, detail)
values
('A0001', 'invalid_bid_amount', 'Bid amount must be greater than highest bid');

-- Policies

create policy live_comment_create
on live.comment for insert to bidder
with check (author_id = current_setting('auth.person_id')::id);

create policy live_comment_read
on live.comment for select to reader
using (true);

create policy live_show_create
on live.show for insert to seller
with check (creator_id = current_setting('auth.person_id')::id);

create policy live_show_read
on live.show for select to reader
using (true);

create policy shop_bid_create
on shop.bid for insert to bidder
with check (bidder_id = current_setting('auth.person_id')::id);

create policy shop_bid_read
on shop.bid for select to reader
using (true);

-- Functions

create function raise_custom_error(error_message text) returns void as $$
declare error conf.error;
begin
  select * into error
  from conf.error
  where message = error_message;

  raise exception using
    errcode = error.code,
    message = error.message,
    detail = error.detail;
end;
$$ language plpgsql;

create function check_bid_amount() returns trigger as $$
declare max_amount amount;
begin
  select max(amount) into max_amount
  from shop.bid
  where bid.auction_id = new.auction_id;

  if new.amount <= max_amount then
    perform raise_custom_error('invalid_bid_amount');
  end if;

  return new;
end;
$$ language plpgsql;

-- Triggers

create trigger check_bid_amount_trigger
before insert on shop.bid for each row
execute function check_bid_amount();
