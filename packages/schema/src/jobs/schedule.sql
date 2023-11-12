-- Function: jobs.schedule()

create function jobs.schedule() returns void
language plpgsql as $$
begin
  perform cron.schedule(
    'check_expired_auctions', '*/20 * * * *', 'select jobs.check_expired_auctions()');
end; $$;
