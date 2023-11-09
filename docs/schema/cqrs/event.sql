-- Enum: cqrs.event_type

create type cqrs.event_type as enum (
  'auction_created',
  'bid_created',
  'comment_created',
  'person_created',
  'product_created',
  'show_created',
  'show_started'
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
