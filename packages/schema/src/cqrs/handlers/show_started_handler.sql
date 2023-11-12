-- Handler: cqrs.show_started_handler

create function cqrs.show_started_handler(event cqrs.show_started) returns void
language plpgsql as $$
declare
  config shop.config;
  show live.show;
begin
  update live.show
  set
    started_at = clock_timestamp(),
    started = not started
  where id = event.id
  returning * into strict show;
end; $$;
