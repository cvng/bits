--
-- Policies
--

-- Table: cqrs.event

create policy event_select_policy on cqrs.event for select to viewer
using ('admin'::auth.role = auth.role());

create policy event_insert_policy on cqrs.event for insert to viewer
with check (true);
