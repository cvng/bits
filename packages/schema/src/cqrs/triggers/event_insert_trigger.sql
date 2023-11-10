-- Trigger: cqrs.event_insert_trigger

create function cqrs.event_insert_trigger() returns trigger
language plpgsql as $$
begin
  perform cqrs.event_insert_handler(new);

  return new;
end; $$;

create trigger event_insert_trigger after insert on cqrs.event
for each row execute function cqrs.event_insert_trigger();
