-- Table: shop.bid

create table shop.bid (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  auction_id id not null references shop.auction (id),
  bidder_id id not null references auth.person (id),
  amount amount not null,
  concurrent_amount amount not null,
  auction_expires_at timestamptz not null
);

alter table shop.bid enable row level security;

-- Check: bid_concurrent_amount_check

alter table shop.bid add constraint bid_concurrent_amount_check
check (amount > concurrent_amount);

-- Check: bid_auction_expires_at_check

alter table shop.bid add constraint bid_auction_expires_at_check
check (created < auction_expires_at);
