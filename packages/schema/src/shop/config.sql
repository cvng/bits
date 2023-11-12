-- Table: shop.config

create table shop.config (
  auction_timeout_secs interval not null default '1 secs',
  auction_refresh_secs interval not null default '1 secs'
);

alter table shop.config enable row level security;
