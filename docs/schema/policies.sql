-- Policies

--
-- Table: auth.person
--

-- Policy: person_select_policy

create policy person_select_policy on auth.person for select to viewer
using (true);

-- Policy: person_insert_policy

create policy person_insert_policy on auth.person for insert to admin
with check (true);

--
-- Table: cqrs.event
--

-- Policy: event_select_policy

create policy event_select_policy on cqrs.event for select to viewer
using ('admin'::auth.role = auth.role());

-- Policy: event_insert_policy

create policy event_insert_policy on cqrs.event for insert to viewer
with check (true);

--
-- Table: live.comment
--

-- Policy: comment_select_policy

create policy comment_select_policy on live.comment for select to viewer
using (true);

-- Policy: comment_insert_policy

create policy comment_insert_policy on live.comment for insert to bidder
with check (author_id = auth.user());

--
-- Table: live.show
--

-- Policy: show_select_policy

create policy show_select_policy on live.show for select to viewer
using (true);

-- Policy: show_insert_policy

create policy show_insert_policy on live.show for insert to seller
with check (creator_id = auth.user());

-- Policy: show_update_policy

create policy show_update_policy on live.show for update to seller
using (true) with check (creator_id = auth.user());

--
-- Table: shop.auction_session
--

-- Policy: shop.auction_session_select_policy

create policy auction_session_select_policy
on shop.auction_session for select to bidder using (true);

-- Policy: shop.auction_session_insert_policy

create policy auction_session_insert_policy
on shop.auction_session for insert to seller with check (true);

-- Policy: shop.auction_session_update_policy

create policy auction_session_update_policy
on shop.auction_session for update to bidder using (true);

--
-- Table: shop.auction
--

-- Policy: auction_select_policy

create policy auction_select_policy on shop.auction for select to viewer
using (true);

-- Policy: auction_insert_policy

create policy auction_insert_policy on shop.auction for insert to seller
with check (
  show_id in (select id from live.show where creator_id = auth.user()) and
  product_id in (select id from shop.product where creator_id = auth.user())
);

-- Policy: auction_update_policy

create policy auction_update_policy on shop.auction for update to seller
using (true) with check (
  show_id in (select id from live.show where creator_id = auth.user()) and
  product_id in (select id from shop.product where creator_id = auth.user())
);

--
-- Table: shop.bid
--

-- Policy: bid_select_policy

create policy bid_select_policy on shop.bid for select to viewer
using (true);

-- Policy: bid_insert_policy

create policy bid_insert_policy on shop.bid for insert to bidder
with check (bidder_id = auth.user());

--
-- Table: shop.config
--

-- Policy: config_select_policy

create policy config_select_policy on shop.config for select to seller
using (true);

--
-- Table: shop.product
--

-- Policy: product_select_policy

create policy product_select_policy on shop.product for select to viewer
using (true);

-- Policy: product_insert_policy

create policy product_insert_policy on shop.product for insert to seller
with check (creator_id = auth.user());
