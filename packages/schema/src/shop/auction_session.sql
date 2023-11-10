-- Table: shop.auction_session

create table shop.auction_session (
  id id not null primary key,
  auction_id id not null unique references shop.auction (id),
  created timestamptz not null default clock_timestamp(),
  max_amount amount not null default 0,
  timeout_secs interval not null,
  refresh_secs interval not null,
  expires_at timestamptz not null
);

alter table shop.auction_session enable row level security;
