-- Table: shop.bid

create table shop.bid (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  auction_id id not null references shop.auction (id),
  bidder_id id not null references auth.person (id),
  amount amount not null,
  concurrent_amount amount not null
);

alter table shop.bid enable row level security;

-- Check: bid_concurrent_amount_check

alter table shop.bid add constraint bid_concurrent_amount_check
check (amount > concurrent_amount);

-- Policy: bid_select_policy

create policy bid_select_policy on shop.bid for select to viewer
using (true);

-- Policy: bid_insert_policy

create policy bid_insert_policy on shop.bid for insert to bidder
with check (bidder_id = auth.user());
