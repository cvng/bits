--
-- Policies
--

-- Table: shop.product

create policy product_select_policy on shop.product for select to viewer
using (true);

create policy product_insert_policy on shop.product for insert to seller
with check (creator_id = auth.user());

-- Table: shop.auction

create policy auction_select_policy on shop.auction for select to viewer
using (true);

create policy auction_insert_policy on shop.auction for insert to seller
with check (
  show_id in (select id from live.show where creator_id = auth.user()) and
  product_id in (select id from shop.product where creator_id = auth.user())
);

create policy auction_update_policy on shop.auction for update to seller
using (true) with check (
  show_id in (select id from live.show where creator_id = auth.user()) and
  product_id in (select id from shop.product where creator_id = auth.user())
);

-- Table: shop.bid

create policy bid_select_policy on shop.bid for select to viewer
using (true);

create policy bid_insert_policy on shop.bid for insert to bidder
with check (bidder_id = auth.user());

-- Table: shop.config

create policy config_select_policy on shop.config for select to seller
using (true);
