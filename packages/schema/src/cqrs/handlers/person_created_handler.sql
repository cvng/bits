-- Handler: cqrs.person_created_handler

create function cqrs.person_created_handler(event cqrs.person_created) returns void
language plpgsql as $$
begin
  insert into auth.person (id, email, role)
  values (event.id, event.email, event.role);
end; $$;
