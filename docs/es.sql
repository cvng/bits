-- CQRS / ES https://github.com/cvng/bits/tree/main/docs/es.sql (work@cvng.dev)

create schema cqrs;

--
-- Enums
--

create type cqrs.event_type as enum (
  'person_created',
  'show_created',
  'product_created',
  'auction_created',
  'bid_created'
);

--
-- Tables
--

create table cqrs.event (
  id id not null default gen_random_uuid() primary key,
  created timestamp not null default now(),
  type cqrs.event_type not null,
  data jsonb not null
);

alter table cqrs.event enable row level security;

--
-- Triggers
--

create function cqrs.event_insert_trigger() returns trigger as $$
begin
  case new.type
    when 'person_created' then perform cqrs.person_created(new);
    /*
    when 'show_created' then
      perform cqrs.on_show_created((new.data)::show_created);
    when 'product_created' then
      perform cqrs.on_product_created((new.data)::product_created);
    when 'auction_created' then
      perform cqrs.on_auction_created((new.data)::auction_created);
    when 'bid_created' then
      perform cqrs.on_bid_created((new.data)::bid_created);
    */
  end case;

  return new;
end;
$$ language plpgsql;

create trigger event_insert_trigger after insert on cqrs.event
for each row execute function cqrs.event_insert_trigger();

--
-- Handlers
--

create function cqrs.person_created(event cqrs.event) returns void as $$
begin
  insert into auth.person (id, email)
  values (
    (event.data->>'id')::id,
    (event.data->>'email')::email
  );
end;
$$ language plpgsql;
