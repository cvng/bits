-- Enum: auth.role

create type auth.role as enum (
  'admin',
  'buyer',
  'seller',
  'anonymous'
);

-- Table: auth.person

create table auth.person (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  email email not null unique,
  role auth.role not null default 'anonymous'::auth.role
);

alter table auth.person enable row level security;
