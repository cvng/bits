-- Table: shop.product

create table shop.product (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  creator_id id not null references auth.person (id),
  name text not null
);

alter table shop.product enable row level security;
