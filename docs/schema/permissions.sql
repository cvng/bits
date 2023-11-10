-- Permissions

grant viewer to bidder;
grant bidder to seller;
grant seller to admin;

grant usage on schema auth to viewer;
grant usage on schema cqrs to viewer;
grant usage on schema live to bidder;
grant usage on schema shop to bidder;

grant select on auth.person to viewer;
grant insert on auth.person to admin;

grant select on cqrs.event to viewer;
grant insert on cqrs.event to viewer;

grant select on live.comment to viewer;
grant insert on live.comment to bidder;

grant select on live.show to viewer;
grant insert on live.show to seller;
grant update on live.show to seller;

grant select on shop.auction to viewer;
grant insert on shop.auction to seller;
grant update on shop.auction to seller;

grant select on shop.auction_session to bidder;
grant insert on shop.auction_session to seller;
grant update on shop.auction_session to bidder;

grant select on shop.bid to viewer;
grant insert on shop.bid to bidder;

grant select on shop.config to seller;

grant select on shop.product to viewer;
grant insert on shop.product to seller;
