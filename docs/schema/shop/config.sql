-- Table: shop.config

create table shop.config (
  auction_timeout_secs interval not null default '60',
  auction_refresh_secs interval not null default '15'
);

alter table shop.config enable row level security;
