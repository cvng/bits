-- Enum: auth.role

create type auth.role as enum (
  'admin',
  'bidder',
  'seller',
  'viewer'
);

-- Table: auth.person

create table auth.person (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  email email not null unique,
  role auth.role not null default 'viewer'::auth.role
);

alter table auth.person enable row level security;

-- Policy: person_select_policy

create policy person_select_policy on auth.person for select to viewer
using (true);

-- Policy: person_insert_policy

create policy person_insert_policy on auth.person for insert to admin
with check (true);
