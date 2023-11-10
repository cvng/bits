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
