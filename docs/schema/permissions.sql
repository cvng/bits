--
-- Permissions
--

-- Hierarchy

grant viewer to bidder;
grant bidder to seller;
grant seller to admin;

-- Schema

grant usage on schema auth to viewer;
grant usage on schema cqrs to viewer;
grant usage on schema live to bidder;
grant usage on schema shop to bidder;

-- Table: cqrs.event

grant select on cqrs.event to viewer;
grant insert on cqrs.event to viewer;

-- Table: auth.person

grant select on auth.person to viewer;
grant insert on auth.person to admin;

-- Table: live.comment

grant select on live.comment to viewer;
grant insert on live.comment to bidder;

-- Table: shop.bid

grant select on shop.bid to viewer;
grant insert on shop.bid to bidder;

-- Table: live.show

grant select on live.show to viewer;
grant insert on live.show to seller;
grant update on live.show to seller;

-- Table: shop.auction

grant select on shop.auction to viewer;
grant insert on shop.auction to seller;
grant update on shop.auction to seller;

-- Table: shop.config

grant select on shop.config to seller;

-- Table: shop.product

grant select on shop.product to viewer;
grant insert on shop.product to seller;
