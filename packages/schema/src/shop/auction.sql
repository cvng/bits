-- Table: shop.auction

create table shop.auction (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  show_id id not null references live.show (id),
  product_id id not null references shop.product (id),
  started_at timestamptz,
  expired_at timestamptz,

  unique (show_id, product_id)
);

alter table shop.auction enable row level security;
