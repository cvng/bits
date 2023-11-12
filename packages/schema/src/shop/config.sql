-- Table: shop.config

create table shop.config (
  auction_timeout_secs interval not null default '60 seconds',
  auction_refresh_secs interval not null default '15 seconds'
);

alter table shop.config enable row level security;
