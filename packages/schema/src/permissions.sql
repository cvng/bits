-- Permissions

grant usage on schema auth to anonymous;
grant usage on schema cqrs to anonymous;
grant usage on schema jobs to admin;
grant usage on schema live to buyer;
grant usage on schema shop to buyer;

-- Table: auth.person

grant select on auth.person to anonymous;
grant insert on auth.person to admin;

-- Table: cqrs.event

grant select on cqrs.event to anonymous;
grant insert on cqrs.event to anonymous;

-- Table: live.comment

grant select on live.comment to anonymous;
grant insert on live.comment to buyer;

-- Table: live.show

grant select on live.show to anonymous;
grant insert on live.show to seller;
grant update on live.show to seller;

-- Table: shop.auction

grant select on shop.auction to anonymous;
grant insert on shop.auction to seller;
grant update on shop.auction to seller;

-- Table: shop.auction_session

grant select on shop.auction_session to buyer;
grant insert on shop.auction_session to seller;
grant update on shop.auction_session to buyer;

-- Table: shop.bid

grant select on shop.bid to anonymous;
grant insert on shop.bid to buyer;

-- Table: shop.config

grant select on shop.config to seller;
grant update on shop.config to admin;

-- Table: shop.product

grant select on shop.product to anonymous;
grant insert on shop.product to seller;

-- Views

grant select on all tables in schema public to anonymous;
