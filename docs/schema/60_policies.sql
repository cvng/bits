--
-- Policies
--

-- Table: cqrs.event

create policy event_select_policy on cqrs.event for select to viewer
using ('admin'::auth.role = auth.role());

create policy event_insert_policy on cqrs.event for insert to viewer
with check (true);

-- Table: auth.person

create policy person_select_policy on auth.person for select to viewer
using (true);

create policy person_insert_policy on auth.person for insert to admin
with check (true);

-- Table: live.show

create policy show_select_policy on live.show for select to viewer
using (true);

create policy show_insert_policy on live.show for insert to seller
with check (creator_id = auth.user());

create policy show_update_policy on live.show for update to seller
using (true) with check (creator_id = auth.user());

-- Table: live.comment

create policy comment_select_policy on live.comment for select to viewer
using (true);

create policy comment_insert_policy on live.comment for insert to bidder
with check (author_id = auth.user());

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
