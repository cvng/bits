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


-- Policy: auction_select_policy

create policy auction_select_policy on shop.auction for select to viewer
using (true);

-- Policy: auction_insert_policy

create policy auction_insert_policy on shop.auction for insert to seller
with check (
  show_id in (select id from live.show where creator_id = auth.user()) and
  product_id in (select id from shop.product where creator_id = auth.user())
);

-- Policy: auction_update_policy

create policy auction_update_policy on shop.auction for update to seller
using (true) with check (
  show_id in (select id from live.show where creator_id = auth.user()) and
  product_id in (select id from shop.product where creator_id = auth.user())
);
