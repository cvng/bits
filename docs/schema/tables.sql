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
  auction_timeout_secs interval not null default '60',
  auction_refresh_secs interval not null default '15'
);

alter table shop.config enable row level security;
