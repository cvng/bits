--
-- Types
--

create type cqrs.auction_created as (
  id id,
  show_id id,
  product_id id
);

create type cqrs.bid_created as (
  id id,
  auction_id id,
  bidder_id id,
  amount amount
);

create type cqrs.comment_created as (
  id id,
  author_id id,
  show_id id,
  text text
);

create type cqrs.person_created as (
  id id,
  email email,
  role auth.role
);

create type cqrs.product_created as (
  id id,
  creator_id id,
  name text
);

create type cqrs.show_created as (
  id id,
  creator_id id,
  name text
);

create type cqrs.show_started as (
  id id
);
