-- Enum: cqrs.event_type

create type cqrs.event_type as enum (
  'auction_created',
  'auction_expired',
  'auction_started',
  'bid_created',
  'comment_created',
  'config_updated',
  'person_created',
  'product_created',
  'show_created',
  'show_started'
);

-- Type: cqrs.auction_created

create type cqrs.auction_created as (
  id id,
  show_id id,
  product_id id
);

-- Type: cqrs.auction_expired

create type cqrs.auction_expired as (
  id id,
  expired_at timestamptz
);

-- Type: cqrs.auction_started

create type cqrs.auction_started as (
  id id
);

-- Type: cqrs.bid_created

create type cqrs.bid_created as (
  id id,
  auction_id id,
  buyer_id id,
  amount amount
);

-- Type: cqrs.comment_created

create type cqrs.comment_created as (
  id id,
  author_id id,
  show_id id,
  text text
);

-- Type: cqrs.config_updated

create type cqrs.config_updated as (
  auction_timeout_secs interval,
  auction_refresh_secs interval
);

-- Type: cqrs.person_created

create type cqrs.person_created as (
  id id,
  email email,
  role auth.role
);

-- Type: cqrs.product_created

create type cqrs.product_created as (
  id id,
  creator_id id,
  name text
);

-- Type: cqrs.show_created

create type cqrs.show_created as (
  id id,
  creator_id id,
  name text
);

-- Type: cqrs.show_started

create type cqrs.show_started as (
  id id
);

-- Table: cqrs.event

create table cqrs.event (
  id bigint not null primary key generated always as identity,
  created timestamptz not null default clock_timestamp(),
  user_id id not null references auth.person (id),
  type cqrs.event_type not null,
  data jsonb not null
);

alter table cqrs.event enable row level security;
