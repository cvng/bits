-- Table: shop.product

create table shop.product (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  creator_id id not null references auth.person (id),
  name text not null
);

alter table shop.product enable row level security;

-- Policy: product_select_policy

create policy product_select_policy on shop.product for select to viewer
using (true);

-- Policy: product_insert_policy

create policy product_insert_policy on shop.product for insert to seller
with check (creator_id = auth.user());
