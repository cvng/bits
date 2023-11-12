-- Function: jobs.schedule()

create function jobs.schedule() returns void
language plpgsql as $$
begin
  perform cron.schedule(
    'check-expired-auctions', '2 seconds', 'select jobs.check_expired_auctions()');
end; $$;
