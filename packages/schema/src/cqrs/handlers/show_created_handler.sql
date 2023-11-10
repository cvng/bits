-- Handler: cqrs.show_created_handler

create function cqrs.show_created_handler(event cqrs.show_created) returns void
language plpgsql as $$
begin
  insert into live.show (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end; $$;
