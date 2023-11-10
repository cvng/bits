-- Handler: cqrs.product_created_handler

create function cqrs.product_created_handler(event cqrs.product_created) returns void
language plpgsql as $$
begin
  insert into shop.product (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end; $$;
