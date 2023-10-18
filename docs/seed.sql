truncate table auth.person cascade;
truncate table live.show cascade;
truncate table shop.auction cascade;
truncate table shop.bid cascade;
truncate table shop.product cascade;

insert into auth.person (id, username)
values
(
  '00000000-0000-0000-0000-000000000000',
  'username@test.dev'
);

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
  100
);
