truncate table auth.person cascade;
truncate table live.comment cascade;
truncate table live.show cascade;
truncate table shop.auction cascade;
truncate table shop.bid cascade;
truncate table shop.product cascade;

insert into cqrs.person_created (id, email)
values
(
  '00000000-0000-0000-0000-000000000000',
  'username@test.dev'
);

insert into cqrs.show_created (id, creator_id, name)
values
(
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  'live_show'
);

insert into cqrs.product_created (id, name)
values
(
  '00000000-0000-0000-0000-000000000000',
  'shop_product'
);

insert into cqrs.auction_created (id, show_id, product_id)
values
(
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000'
);

insert into cqrs.bid_created (id, auction_id, bidder_id, amount)
values
(
  'cd265921-a49d-4650-ac7b-f8614248f147',
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  200
);

insert into cqrs.bid_created (id, auction_id, bidder_id, amount)
values
(
  '3f2d88d3-9318-4623-8c5b-4d4675318ffd',
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  300
);

insert into cqrs.bid_created (id, auction_id, bidder_id, amount)
values
(
  '130ad3e1-38ab-4220-8119-2a7872ae7baa',
  '00000000-0000-0000-0000-000000000000',
  '00000000-0000-0000-0000-000000000000',
  400
);
