-- Handler: cqrs.auction_created_handler

create function cqrs.auction_created_handler(event cqrs.auction_created) returns void
language plpgsql as $$
declare
  config shop.config;
begin
  select * from shop.config() into config;

  insert into shop.auction (id, show_id, product_id)
  values (
    event.id,
    event.show_id,
    event.product_id
  );
end; $$;
