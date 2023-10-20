truncate table auth.person cascade;
truncate table live.comment cascade;
truncate table live.show cascade;
truncate table shop.auction cascade;
truncate table shop.bid cascade;
truncate table shop.product cascade;

insert into cqrs.event (type, data)
values
('person_created', '{ "id": "00000000-0000-0000-0000-000000000000", "email": "username@test.dev" }'); -- noqa: LT05

insert into live.show (id, creator_id, name)
values
(
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  'live_show'
);

insert into shop.product (id, name)
values
(
  '00000000-0000-0000-0000-000000000000',
  'shop_product'
);

insert into shop.auction (id, show_id, product_id)
values
(
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000'
);

insert into shop.bid (auction_id, bidder_id, amount)
values
(
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  200
);

insert into shop.bid (auction_id, bidder_id, amount)
values
(
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  300
);

insert into shop.bid (auction_id, bidder_id, amount)
values
(
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  400
);
