-- Util: cqrs.notify()

create function cqrs.notify(event cqrs.event) returns cqrs.event
language plpgsql as $$
begin
  perform pg_notify(
    'cqrs.event',
    jsonb_build_object(
      'id', event.id,
      'created', event.created,
      'user_id', event.user_id,
      'type', event.type,
      'data', event.data
    )::text
  );

  return event;
end; $$;
