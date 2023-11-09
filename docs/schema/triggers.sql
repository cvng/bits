--
-- Triggers
--

-- Trigger: cqrs.event_insert_trigger

create function cqrs.event_insert_trigger() returns trigger
language plpgsql as $$
begin
  perform auth.login(new.user_id);

  case new.type
    when 'auction_created' then
      perform cqrs.auction_created_handler(
        jsonb_populate_record(null::cqrs.auction_created, new.data));

    when 'bid_created' then
      perform cqrs.bid_created_handler(
        jsonb_populate_record(null::cqrs.bid_created, new.data));

    when 'comment_created' then
      perform cqrs.comment_created_handler(
        jsonb_populate_record(null::cqrs.comment_created, new.data));

    when 'person_created' then
      perform cqrs.person_created_handler(
        jsonb_populate_record(null::cqrs.person_created, new.data));

    when 'product_created' then
      perform cqrs.product_created_handler(
        jsonb_populate_record(null::cqrs.product_created, new.data));

    when 'show_created' then
      perform cqrs.show_created_handler(
        jsonb_populate_record(null::cqrs.show_created, new.data));

    when 'show_started' then
      perform cqrs.show_started_handler(
        jsonb_populate_record(null::cqrs.show_started, new.data));
  end case;

  perform cqrs.handler(new);

  return new;
end;
$$;

create trigger event_insert_trigger after insert on cqrs.event
for each row execute function cqrs.event_insert_trigger();
