-- Views

create view public.person with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    email
  from auth.person
);

create view public.show with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    creator_id,
    name,
    started_at,
    started
  from live.show
);

create view public.comment with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    author_id,
    show_id,
    text
  from live.comment
);

create view public.product with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    creator_id,
    name
  from shop.product
);

create view public.auction with (security_invoker = true) as (
  select
    id,
    created,
    updated,
    show_id,
    product_id,
    started_at,
    expired_at
  from shop.auction
);

create view public.bid with (security_invoker = true) as (
  select
    id,
    created,
    auction_id,
    buyer_id,
    concurrent_amount,
    amount
  from shop.bid
);
