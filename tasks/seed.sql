truncate table auth.person cascade;
truncate table live.comment cascade;
truncate table live.show cascade;
truncate table shop.auction cascade;
truncate table shop.bid cascade;
truncate table shop.product cascade;

insert into cqrs.event (type, data)
values
-- noqa: disable=LT05
('person_created', '{"id": "00000000-0000-0000-0000-000000000000", "email": "username@test.dev"}'),
('show_created', '{"id": "00000000-0000-0000-0000-000000000000", "creator_id": "00000000-0000-0000-0000-000000000000", "name": "live_show"}'),
('product_created', '{"id": "00000000-0000-0000-0000-000000000000", "name": "shop_product"}'),
('auction_created', '{"id": "00000000-0000-0000-0000-000000000000", "show_id": "00000000-0000-0000-0000-000000000000", "product_id": "00000000-0000-0000-0000-000000000000"}'),
('bid_created', '{"auction_id": "00000000-0000-0000-0000-000000000000", "bidder_id": "00000000-0000-0000-0000-000000000000", "amount": 200}'),
('bid_created', '{"auction_id": "00000000-0000-0000-0000-000000000000", "bidder_id": "00000000-0000-0000-0000-000000000000", "amount": 300}'),
('bid_created', '{"auction_id": "00000000-0000-0000-0000-000000000000", "bidder_id": "00000000-0000-0000-0000-000000000000", "amount": 400}');
-- noqa: enable=LT05
