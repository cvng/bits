-- Table: live.comment

create table live.comment (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  author_id id not null references auth.person (id),
  show_id id not null references live.show (id),
  text text not null
);

alter table live.comment enable row level security;

-- Policy: comment_select_policy

create policy comment_select_policy on live.comment for select to viewer
using (true);

-- Policy: comment_insert_policy

create policy comment_insert_policy on live.comment for insert to bidder
with check (author_id = auth.user());
