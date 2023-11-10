-- Table: live.show

create table live.show (
  id id not null primary key,
  created timestamptz not null default clock_timestamp(),
  updated timestamptz,
  creator_id id not null references auth.person (id),
  name text not null,
  started_at timestamptz,
  started boolean default false
);

alter table live.show enable row level security;

-- Check: show_already_started_check

alter table live.show add constraint show_already_started_check
check (
  (started_at is null and not started) or
  (started_at is not null and started)
);
