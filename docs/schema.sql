-- Extensions.

create extension pgcrypto;

-- Domains.

create domain text_min as text check(value != '');

-- Tables.

create table show (
    id uuid not null default gen_random_uuid() primary key,
    name text_min not null
);

alter table show enable row level security;
