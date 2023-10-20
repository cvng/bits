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
    when 'auction_created' then perform cqrs.auction_created(new);
    when 'bid_created' then perform cqrs.bid_created(new);
    when 'person_created' then perform cqrs.person_created(new);
    when 'product_created' then perform cqrs.product_created(new);
    when 'show_created' then perform cqrs.show_created(new);
  end case;

  return new;
end;
$$ language plpgsql;

create trigger event_insert_trigger after insert on cqrs.event
for each row execute function cqrs.event_insert_trigger();

--
-- Handlers
--

create function cqrs.auction_created(event cqrs.event) returns void as $$
begin
  insert into shop.auction (id, show_id, product_id)
  values (
    (event.data->>'id')::id,
    (event.data->>'show_id')::id,
    (event.data->>'product_id')::id
  );
end;
$$ language plpgsql;

create function cqrs.bid_created(event cqrs.event) returns void as $$
begin
  insert into shop.bid (id, auction_id, bidder_id, amount)
  values (
    (event.data->>'id')::id,
    (event.data->>'auction_id')::id,
    (event.data->>'bidder_id')::id,
    (event.data->>'amount')::amount
  );
end;
$$ language plpgsql;

create function cqrs.person_created(event cqrs.event) returns void as $$
begin
  insert into auth.person (id, email)
  values (
    (event.data->>'id')::id,
    (event.data->>'email')::email
  );
end;
$$ language plpgsql;

create function cqrs.product_created(event cqrs.event) returns void as $$
begin
  insert into shop.product (id, name)
  values (
    (event.data->>'id')::id,
    (event.data->>'name')::text
  );
end;
$$ language plpgsql;

create function cqrs.show_created(event cqrs.event) returns void as $$
begin
  insert into live.show (id, creator_id, name)
  values (
    (event.data->>'id')::id,
    (event.data->>'creator_id')::id,
    (event.data->>'name')::text
  );
end;
$$ language plpgsql;
