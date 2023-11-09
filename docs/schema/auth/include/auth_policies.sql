--
-- Policies
--

-- Table: auth.person

create policy person_select_policy on auth.person for select to viewer
using (true);

create policy person_insert_policy on auth.person for insert to admin
with check (true);
