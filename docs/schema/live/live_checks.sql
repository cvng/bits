--
-- Checks
--

-- Check: show_already_started_check

alter table live.show add constraint show_already_started_check
check (
  (started_at is null and not started) or
  (started_at is not null and started)
);
