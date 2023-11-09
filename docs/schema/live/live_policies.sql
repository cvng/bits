--
-- Policies
--

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
