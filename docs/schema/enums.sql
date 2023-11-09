--
-- Enums
--

create type auth.role as enum (
  'admin',
  'bidder',
  'seller',
  'viewer'
);

create type cqrs.event_type as enum (
  'auction_created',
  'bid_created',
  'comment_created',
  'person_created',
  'product_created',
  'show_created',
  'show_started'
);
