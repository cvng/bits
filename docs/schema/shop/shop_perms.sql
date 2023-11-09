--
-- Permissions
--

-- Table: shop.bid

grant select on shop.bid to viewer;
grant insert on shop.bid to bidder;

-- Table: shop.auction

grant select on shop.auction to viewer;
grant insert on shop.auction to seller;
grant update on shop.auction to seller;

-- Table: shop.config

grant select on shop.config to seller;

-- Table: shop.product

grant select on shop.product to viewer;
grant insert on shop.product to seller;
