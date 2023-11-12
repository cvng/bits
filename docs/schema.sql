--
-- PostgreSQL database dump
--

-- Dumped from database version 16.0 (Debian 16.0-1.pgdg120+1)
-- Dumped by pg_dump version 16.1 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: auth; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA auth;


--
-- Name: cqrs; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA cqrs;


--
-- Name: pg_cron; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pg_cron WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION pg_cron; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION pg_cron IS 'Job scheduler for PostgreSQL';


--
-- Name: jobs; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA jobs;


--
-- Name: live; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA live;


--
-- Name: shop; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA shop;


--
-- Name: role; Type: TYPE; Schema: auth; Owner: -
--

CREATE TYPE auth.role AS ENUM (
    'admin',
    'buyer',
    'seller',
    'anonymous'
);


--
-- Name: id; Type: DOMAIN; Schema: public; Owner: -
--

CREATE DOMAIN public.id AS uuid
	CONSTRAINT id_check CHECK ((VALUE IS NOT NULL));


--
-- Name: auction_created; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.auction_created AS (
	id public.id,
	show_id public.id,
	product_id public.id
);


--
-- Name: auction_expired; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.auction_expired AS (
	id public.id,
	expired_at timestamp with time zone
);


--
-- Name: auction_started; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.auction_started AS (
	id public.id
);


--
-- Name: amount; Type: DOMAIN; Schema: public; Owner: -
--

CREATE DOMAIN public.amount AS numeric
	CONSTRAINT amount_check CHECK ((VALUE >= (0)::numeric));


--
-- Name: bid_created; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.bid_created AS (
	id public.id,
	auction_id public.id,
	buyer_id public.id,
	amount public.amount
);


--
-- Name: comment_created; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.comment_created AS (
	id public.id,
	author_id public.id,
	show_id public.id,
	text text
);


--
-- Name: config_updated; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.config_updated AS (
	auction_timeout_secs interval,
	auction_refresh_secs interval
);


--
-- Name: event_type; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.event_type AS ENUM (
    'auction_created',
    'auction_expired',
    'auction_started',
    'bid_created',
    'comment_created',
    'config_updated',
    'person_created',
    'product_created',
    'show_created',
    'show_started'
);


--
-- Name: email; Type: DOMAIN; Schema: public; Owner: -
--

CREATE DOMAIN public.email AS text
	CONSTRAINT email_check CHECK (((VALUE = lower(VALUE)) AND (VALUE ~~ '%@%'::text)));


--
-- Name: person_created; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.person_created AS (
	id public.id,
	email public.email,
	role auth.role
);


--
-- Name: product_created; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.product_created AS (
	id public.id,
	creator_id public.id,
	name text
);


--
-- Name: show_created; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.show_created AS (
	id public.id,
	creator_id public.id,
	name text
);


--
-- Name: show_started; Type: TYPE; Schema: cqrs; Owner: -
--

CREATE TYPE cqrs.show_started AS (
	id public.id
);


--
-- Name: admin(); Type: FUNCTION; Schema: auth; Owner: -
--

CREATE FUNCTION auth.admin() RETURNS public.id
    LANGUAGE plpgsql
    AS $$
begin
  return '00000000-0000-0000-0000-000000000000'::id;
end; $$;


--
-- Name: login(public.id); Type: FUNCTION; Schema: auth; Owner: -
--

CREATE FUNCTION auth.login(user_id public.id) RETURNS auth.role
    LANGUAGE plpgsql
    AS $$
declare
  enabled_role auth.role;
begin
  select role into enabled_role from auth.person where id = user_id;

  if enabled_role is not null then
    perform set_config('role', enabled_role::text, true); -- set local role %I
    perform set_config('auth.user', user_id::text, true);
  else
    perform set_config('role', 'anonymous'::auth.role::text, true);
  end if;

  return auth.role();
end; $$;


--
-- Name: logout(); Type: FUNCTION; Schema: auth; Owner: -
--

CREATE FUNCTION auth.logout() RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  reset "role";
  reset "auth.user";
end; $$;


--
-- Name: role(); Type: FUNCTION; Schema: auth; Owner: -
--

CREATE FUNCTION auth.role() RETURNS auth.role
    LANGUAGE plpgsql
    AS $$
begin
  return current_setting('role')::auth.role;
end; $$;


--
-- Name: user(); Type: FUNCTION; Schema: auth; Owner: -
--

CREATE FUNCTION auth."user"() RETURNS uuid
    LANGUAGE plpgsql
    AS $$
begin
  begin return current_setting('auth.user')::id;
  exception when undefined_object then return null; end;
end; $$;


--
-- Name: auction_created_handler(cqrs.auction_created); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.auction_created_handler(event cqrs.auction_created) RETURNS void
    LANGUAGE plpgsql
    AS $$
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


--
-- Name: auction_expired_handler(cqrs.auction_expired); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.auction_expired_handler(event cqrs.auction_expired) RETURNS void
    LANGUAGE plpgsql
    AS $$
declare
  auction shop.auction;
begin
  update shop.auction
  set expired_at = event.expired_at
  where id = event.id
  returning * into strict auction;
end; $$;


--
-- Name: auction_started_handler(cqrs.auction_started); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.auction_started_handler(event cqrs.auction_started) RETURNS void
    LANGUAGE plpgsql
    AS $$
declare
  now timestamptz;
  config shop.config;
  auction shop.auction;
  session shop.auction_session;
begin
  select clock_timestamp() into now;
  select * from shop.config() into config;

  insert into shop.auction_session (
    id,
    auction_id,
    max_amount,
    timeout_secs,
    refresh_secs,
    expires_at
  )
  values (
    gen_random_uuid(),
    event.id,
    default,
    config.auction_timeout_secs,
    config.auction_refresh_secs,
    config.auction_timeout_secs + now
  )
   returning * into strict session;

  update shop.auction set started_at = session.created
  where id = event.id returning * into strict auction;
end; $$;


--
-- Name: bid_created_handler(cqrs.bid_created); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.bid_created_handler(event cqrs.bid_created) RETURNS void
    LANGUAGE plpgsql
    AS $$
declare
  bid shop.bid;
  session shop.auction_session;
  new_expires_at timestamptz;
begin
  select * into strict session
  from shop.auction_session where auction_id = event.auction_id;

  insert into shop.bid (
    id,
    auction_id,
    buyer_id,
    amount,
    concurrent_amount,
    auction_expires_at
  )
  values (
    event.id,
    event.auction_id,
    event.buyer_id,
    event.amount,
    session.max_amount,
    session.expires_at
  )
  returning * into strict bid;

  if session.expires_at - bid.created < session.refresh_secs then
    new_expires_at := session.created + session.refresh_secs;
  else
    new_expires_at := session.expires_at;
  end if;

  update shop.auction_session
  set
    max_amount = bid.amount,
    expires_at = new_expires_at
  where id = session.id
  returning * into strict session;
end; $$;


--
-- Name: comment_created_handler(cqrs.comment_created); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.comment_created_handler(event cqrs.comment_created) RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  insert into live.comment (id, author_id, show_id, text)
  values (event.id, event.author_id, event.show_id, event.text);
end; $$;


--
-- Name: config_updated_handler(cqrs.config_updated); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.config_updated_handler(event cqrs.config_updated) RETURNS void
    LANGUAGE plpgsql
    AS $$
declare
  config shop.config;
begin
  update shop.config
  set
    auction_timeout_secs = coalesce(event.auction_timeout_secs, auction_timeout_secs),
    auction_refresh_secs = coalesce(event.auction_refresh_secs, auction_refresh_secs)
  returning * into strict config;
end; $$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: event; Type: TABLE; Schema: cqrs; Owner: -
--

CREATE TABLE cqrs.event (
    id bigint NOT NULL,
    created timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    user_id public.id NOT NULL,
    type cqrs.event_type NOT NULL,
    data jsonb NOT NULL
);


--
-- Name: event_insert_handler(cqrs.event); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.event_insert_handler(event cqrs.event) RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  case event.type
    when 'auction_created' then
      perform cqrs.auction_created_handler(
        jsonb_populate_record(null::cqrs.auction_created, event.data));

   when 'auction_expired' then
      perform cqrs.auction_expired_handler(
        jsonb_populate_record(null::cqrs.auction_expired, event.data));

   when 'auction_started' then
      perform cqrs.auction_started_handler(
        jsonb_populate_record(null::cqrs.auction_started, event.data));

    when 'bid_created' then
      perform cqrs.bid_created_handler(
        jsonb_populate_record(null::cqrs.bid_created, event.data));

    when 'comment_created' then
      perform cqrs.comment_created_handler(
        jsonb_populate_record(null::cqrs.comment_created, event.data));

    when 'config_updated' then
      perform cqrs.config_updated_handler(
        jsonb_populate_record(null::cqrs.config_updated, event.data));

    when 'person_created' then
      perform cqrs.person_created_handler(
        jsonb_populate_record(null::cqrs.person_created, event.data));

    when 'product_created' then
      perform cqrs.product_created_handler(
        jsonb_populate_record(null::cqrs.product_created, event.data));

    when 'show_created' then
      perform cqrs.show_created_handler(
        jsonb_populate_record(null::cqrs.show_created, event.data));

    when 'show_started' then
      perform cqrs.show_started_handler(
        jsonb_populate_record(null::cqrs.show_started, event.data));
  end case;

  perform cqrs.notify(event);
end; $$;


--
-- Name: event_insert_trigger(); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.event_insert_trigger() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
begin
  perform cqrs.event_insert_handler(new);

  return new;
end; $$;


--
-- Name: notify(cqrs.event); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.notify(event cqrs.event) RETURNS cqrs.event
    LANGUAGE plpgsql
    AS $$
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


--
-- Name: person_created_handler(cqrs.person_created); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.person_created_handler(event cqrs.person_created) RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  insert into auth.person (id, email, role)
  values (event.id, event.email, event.role);
end; $$;


--
-- Name: product_created_handler(cqrs.product_created); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.product_created_handler(event cqrs.product_created) RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  insert into shop.product (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end; $$;


--
-- Name: show_created_handler(cqrs.show_created); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.show_created_handler(event cqrs.show_created) RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  insert into live.show (id, creator_id, name)
  values (event.id, event.creator_id, event.name);
end; $$;


--
-- Name: show_started_handler(cqrs.show_started); Type: FUNCTION; Schema: cqrs; Owner: -
--

CREATE FUNCTION cqrs.show_started_handler(event cqrs.show_started) RETURNS void
    LANGUAGE plpgsql
    AS $$
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


--
-- Name: check_expired_auctions(); Type: FUNCTION; Schema: jobs; Owner: -
--

CREATE FUNCTION jobs.check_expired_auctions() RETURNS void
    LANGUAGE plpgsql
    AS $$
declare
  expired_auction shop.auction_session;
begin
  for expired_auction in
    select *
    from shop.auction_session
    where expires_at < clock_timestamp()
  loop
    insert into cqrs.event (user_id, type, data)
    values (
      auth.admin(),
      'auction_expired'::cqrs.event_type,
      json_build_object(
        'id', expired_auction.auction_id,
        'expired_at', expired_auction.expires_at
      )::jsonb
    );
  end loop;
end; $$;


--
-- Name: schedule(); Type: FUNCTION; Schema: jobs; Owner: -
--

CREATE FUNCTION jobs.schedule() RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  perform cron.schedule(
    'check-expired-auctions', '2 seconds', 'select jobs.check_expired_auctions()');
end; $$;


--
-- Name: config; Type: TABLE; Schema: shop; Owner: -
--

CREATE TABLE shop.config (
    auction_timeout_secs interval DEFAULT '00:01:00'::interval NOT NULL,
    auction_refresh_secs interval DEFAULT '00:00:15'::interval NOT NULL
);


--
-- Name: config(); Type: FUNCTION; Schema: shop; Owner: -
--

CREATE FUNCTION shop.config() RETURNS shop.config
    LANGUAGE plpgsql
    AS $$
declare
    config shop.config;
begin
  select * into strict config from shop.config;

  return config;
end; $$;


--
-- Name: person; Type: TABLE; Schema: auth; Owner: -
--

CREATE TABLE auth.person (
    id public.id NOT NULL,
    created timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    updated timestamp with time zone,
    email public.email NOT NULL,
    role auth.role DEFAULT 'anonymous'::auth.role NOT NULL
);


--
-- Name: event_id_seq; Type: SEQUENCE; Schema: cqrs; Owner: -
--

ALTER TABLE cqrs.event ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME cqrs.event_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: comment; Type: TABLE; Schema: live; Owner: -
--

CREATE TABLE live.comment (
    id public.id NOT NULL,
    created timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    updated timestamp with time zone,
    author_id public.id NOT NULL,
    show_id public.id NOT NULL,
    text text NOT NULL
);


--
-- Name: show; Type: TABLE; Schema: live; Owner: -
--

CREATE TABLE live.show (
    id public.id NOT NULL,
    created timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    updated timestamp with time zone,
    creator_id public.id NOT NULL,
    name text NOT NULL,
    started_at timestamp with time zone,
    started boolean DEFAULT false,
    CONSTRAINT show_already_started_check CHECK ((((started_at IS NULL) AND (NOT started)) OR ((started_at IS NOT NULL) AND started)))
);


--
-- Name: auction; Type: TABLE; Schema: shop; Owner: -
--

CREATE TABLE shop.auction (
    id public.id NOT NULL,
    created timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    updated timestamp with time zone,
    show_id public.id NOT NULL,
    product_id public.id NOT NULL,
    started_at timestamp with time zone,
    expired_at timestamp with time zone
);


--
-- Name: auction; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.auction WITH (security_invoker='true') AS
 SELECT id,
    created,
    updated,
    show_id,
    product_id,
    started_at,
    expired_at
   FROM shop.auction;


--
-- Name: bid; Type: TABLE; Schema: shop; Owner: -
--

CREATE TABLE shop.bid (
    id public.id NOT NULL,
    created timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    auction_id public.id NOT NULL,
    buyer_id public.id NOT NULL,
    amount public.amount NOT NULL,
    concurrent_amount public.amount NOT NULL,
    auction_expires_at timestamp with time zone NOT NULL,
    CONSTRAINT bid_auction_expires_at_check CHECK ((created < auction_expires_at)),
    CONSTRAINT bid_concurrent_amount_check CHECK (((amount)::numeric > (concurrent_amount)::numeric))
);


--
-- Name: bid; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.bid WITH (security_invoker='true') AS
 SELECT id,
    created,
    auction_id,
    buyer_id,
    concurrent_amount,
    amount
   FROM shop.bid;


--
-- Name: comment; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.comment WITH (security_invoker='true') AS
 SELECT id,
    created,
    updated,
    author_id,
    show_id,
    text
   FROM live.comment;


--
-- Name: person; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.person WITH (security_invoker='true') AS
 SELECT id,
    created,
    updated,
    email
   FROM auth.person;


--
-- Name: product; Type: TABLE; Schema: shop; Owner: -
--

CREATE TABLE shop.product (
    id public.id NOT NULL,
    created timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    updated timestamp with time zone,
    creator_id public.id NOT NULL,
    name text NOT NULL
);


--
-- Name: product; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.product WITH (security_invoker='true') AS
 SELECT id,
    created,
    updated,
    name
   FROM shop.product;


--
-- Name: show; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.show WITH (security_invoker='true') AS
 SELECT id,
    created,
    updated,
    creator_id,
    name,
    started_at,
    started
   FROM live.show;


--
-- Name: auction_session; Type: TABLE; Schema: shop; Owner: -
--

CREATE TABLE shop.auction_session (
    id public.id NOT NULL,
    auction_id public.id NOT NULL,
    created timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    max_amount public.amount DEFAULT 0 NOT NULL,
    timeout_secs interval NOT NULL,
    refresh_secs interval NOT NULL,
    expires_at timestamp with time zone NOT NULL
);


--
-- Name: person person_email_key; Type: CONSTRAINT; Schema: auth; Owner: -
--

ALTER TABLE ONLY auth.person
    ADD CONSTRAINT person_email_key UNIQUE (email);


--
-- Name: person person_pkey; Type: CONSTRAINT; Schema: auth; Owner: -
--

ALTER TABLE ONLY auth.person
    ADD CONSTRAINT person_pkey PRIMARY KEY (id);


--
-- Name: event event_pkey; Type: CONSTRAINT; Schema: cqrs; Owner: -
--

ALTER TABLE ONLY cqrs.event
    ADD CONSTRAINT event_pkey PRIMARY KEY (id);


--
-- Name: comment comment_pkey; Type: CONSTRAINT; Schema: live; Owner: -
--

ALTER TABLE ONLY live.comment
    ADD CONSTRAINT comment_pkey PRIMARY KEY (id);


--
-- Name: show show_pkey; Type: CONSTRAINT; Schema: live; Owner: -
--

ALTER TABLE ONLY live.show
    ADD CONSTRAINT show_pkey PRIMARY KEY (id);


--
-- Name: auction auction_pkey; Type: CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.auction
    ADD CONSTRAINT auction_pkey PRIMARY KEY (id);


--
-- Name: auction_session auction_session_auction_id_key; Type: CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.auction_session
    ADD CONSTRAINT auction_session_auction_id_key UNIQUE (auction_id);


--
-- Name: auction_session auction_session_pkey; Type: CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.auction_session
    ADD CONSTRAINT auction_session_pkey PRIMARY KEY (id);


--
-- Name: auction auction_show_id_product_id_key; Type: CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.auction
    ADD CONSTRAINT auction_show_id_product_id_key UNIQUE (show_id, product_id);


--
-- Name: bid bid_pkey; Type: CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.bid
    ADD CONSTRAINT bid_pkey PRIMARY KEY (id);


--
-- Name: product product_pkey; Type: CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.product
    ADD CONSTRAINT product_pkey PRIMARY KEY (id);


--
-- Name: event event_insert_trigger; Type: TRIGGER; Schema: cqrs; Owner: -
--

CREATE TRIGGER event_insert_trigger AFTER INSERT ON cqrs.event FOR EACH ROW EXECUTE FUNCTION cqrs.event_insert_trigger();


--
-- Name: event event_user_id_fkey; Type: FK CONSTRAINT; Schema: cqrs; Owner: -
--

ALTER TABLE ONLY cqrs.event
    ADD CONSTRAINT event_user_id_fkey FOREIGN KEY (user_id) REFERENCES auth.person(id);


--
-- Name: comment comment_author_id_fkey; Type: FK CONSTRAINT; Schema: live; Owner: -
--

ALTER TABLE ONLY live.comment
    ADD CONSTRAINT comment_author_id_fkey FOREIGN KEY (author_id) REFERENCES auth.person(id);


--
-- Name: comment comment_show_id_fkey; Type: FK CONSTRAINT; Schema: live; Owner: -
--

ALTER TABLE ONLY live.comment
    ADD CONSTRAINT comment_show_id_fkey FOREIGN KEY (show_id) REFERENCES live.show(id);


--
-- Name: show show_creator_id_fkey; Type: FK CONSTRAINT; Schema: live; Owner: -
--

ALTER TABLE ONLY live.show
    ADD CONSTRAINT show_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES auth.person(id);


--
-- Name: auction auction_product_id_fkey; Type: FK CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.auction
    ADD CONSTRAINT auction_product_id_fkey FOREIGN KEY (product_id) REFERENCES shop.product(id);


--
-- Name: auction_session auction_session_auction_id_fkey; Type: FK CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.auction_session
    ADD CONSTRAINT auction_session_auction_id_fkey FOREIGN KEY (auction_id) REFERENCES shop.auction(id);


--
-- Name: auction auction_show_id_fkey; Type: FK CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.auction
    ADD CONSTRAINT auction_show_id_fkey FOREIGN KEY (show_id) REFERENCES live.show(id);


--
-- Name: bid bid_auction_id_fkey; Type: FK CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.bid
    ADD CONSTRAINT bid_auction_id_fkey FOREIGN KEY (auction_id) REFERENCES shop.auction(id);


--
-- Name: bid bid_buyer_id_fkey; Type: FK CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.bid
    ADD CONSTRAINT bid_buyer_id_fkey FOREIGN KEY (buyer_id) REFERENCES auth.person(id);


--
-- Name: product product_creator_id_fkey; Type: FK CONSTRAINT; Schema: shop; Owner: -
--

ALTER TABLE ONLY shop.product
    ADD CONSTRAINT product_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES auth.person(id);


--
-- Name: person; Type: ROW SECURITY; Schema: auth; Owner: -
--

ALTER TABLE auth.person ENABLE ROW LEVEL SECURITY;

--
-- Name: person person_insert_policy; Type: POLICY; Schema: auth; Owner: -
--

CREATE POLICY person_insert_policy ON auth.person FOR INSERT TO admin WITH CHECK (true);


--
-- Name: person person_select_policy; Type: POLICY; Schema: auth; Owner: -
--

CREATE POLICY person_select_policy ON auth.person FOR SELECT TO anonymous USING (true);


--
-- Name: event; Type: ROW SECURITY; Schema: cqrs; Owner: -
--

ALTER TABLE cqrs.event ENABLE ROW LEVEL SECURITY;

--
-- Name: event event_insert_policy; Type: POLICY; Schema: cqrs; Owner: -
--

CREATE POLICY event_insert_policy ON cqrs.event FOR INSERT TO authenticated WITH CHECK (((user_id)::uuid = auth."user"()));


--
-- Name: event event_select_policy; Type: POLICY; Schema: cqrs; Owner: -
--

CREATE POLICY event_select_policy ON cqrs.event FOR SELECT TO admin USING (true);


--
-- Name: job cron_job_policy; Type: POLICY; Schema: cron; Owner: -
--

CREATE POLICY cron_job_policy ON cron.job USING ((username = CURRENT_USER));


--
-- Name: job_run_details cron_job_run_details_policy; Type: POLICY; Schema: cron; Owner: -
--

CREATE POLICY cron_job_run_details_policy ON cron.job_run_details USING ((username = CURRENT_USER));


--
-- Name: job; Type: ROW SECURITY; Schema: cron; Owner: -
--

ALTER TABLE cron.job ENABLE ROW LEVEL SECURITY;

--
-- Name: job_run_details; Type: ROW SECURITY; Schema: cron; Owner: -
--

ALTER TABLE cron.job_run_details ENABLE ROW LEVEL SECURITY;

--
-- Name: comment; Type: ROW SECURITY; Schema: live; Owner: -
--

ALTER TABLE live.comment ENABLE ROW LEVEL SECURITY;

--
-- Name: comment comment_insert_policy; Type: POLICY; Schema: live; Owner: -
--

CREATE POLICY comment_insert_policy ON live.comment FOR INSERT TO buyer WITH CHECK (((author_id)::uuid = auth."user"()));


--
-- Name: comment comment_select_policy; Type: POLICY; Schema: live; Owner: -
--

CREATE POLICY comment_select_policy ON live.comment FOR SELECT TO anonymous USING (true);


--
-- Name: show; Type: ROW SECURITY; Schema: live; Owner: -
--

ALTER TABLE live.show ENABLE ROW LEVEL SECURITY;

--
-- Name: show show_insert_policy; Type: POLICY; Schema: live; Owner: -
--

CREATE POLICY show_insert_policy ON live.show FOR INSERT TO seller WITH CHECK (((creator_id)::uuid = auth."user"()));


--
-- Name: show show_select_policy; Type: POLICY; Schema: live; Owner: -
--

CREATE POLICY show_select_policy ON live.show FOR SELECT TO anonymous USING (true);


--
-- Name: show show_update_policy; Type: POLICY; Schema: live; Owner: -
--

CREATE POLICY show_update_policy ON live.show FOR UPDATE TO seller USING (true) WITH CHECK (((creator_id)::uuid = auth."user"()));


--
-- Name: auction; Type: ROW SECURITY; Schema: shop; Owner: -
--

ALTER TABLE shop.auction ENABLE ROW LEVEL SECURITY;

--
-- Name: auction auction_insert_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY auction_insert_policy ON shop.auction FOR INSERT TO seller WITH CHECK ((((show_id)::uuid IN ( SELECT show.id
   FROM live.show
  WHERE ((show.creator_id)::uuid = auth."user"()))) AND ((product_id)::uuid IN ( SELECT product.id
   FROM shop.product
  WHERE ((product.creator_id)::uuid = auth."user"())))));


--
-- Name: auction auction_select_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY auction_select_policy ON shop.auction FOR SELECT TO anonymous USING (true);


--
-- Name: auction_session; Type: ROW SECURITY; Schema: shop; Owner: -
--

ALTER TABLE shop.auction_session ENABLE ROW LEVEL SECURITY;

--
-- Name: auction_session auction_session_insert_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY auction_session_insert_policy ON shop.auction_session FOR INSERT TO seller WITH CHECK (((auction_id)::uuid IN ( SELECT auction.id
   FROM shop.auction
  WHERE ((auction.show_id)::uuid IN ( SELECT show.id
           FROM live.show
          WHERE ((show.creator_id)::uuid = auth."user"()))))));


--
-- Name: auction_session auction_session_select_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY auction_session_select_policy ON shop.auction_session FOR SELECT TO anonymous USING (true);


--
-- Name: auction_session auction_session_update_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY auction_session_update_policy ON shop.auction_session FOR UPDATE TO buyer USING (true);


--
-- Name: auction auction_update_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY auction_update_policy ON shop.auction FOR UPDATE TO seller USING (true) WITH CHECK ((((show_id)::uuid IN ( SELECT show.id
   FROM live.show
  WHERE ((show.creator_id)::uuid = auth."user"()))) AND ((product_id)::uuid IN ( SELECT product.id
   FROM shop.product
  WHERE ((product.creator_id)::uuid = auth."user"())))));


--
-- Name: bid; Type: ROW SECURITY; Schema: shop; Owner: -
--

ALTER TABLE shop.bid ENABLE ROW LEVEL SECURITY;

--
-- Name: bid bid_insert_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY bid_insert_policy ON shop.bid FOR INSERT TO buyer WITH CHECK (((buyer_id)::uuid = auth."user"()));


--
-- Name: bid bid_select_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY bid_select_policy ON shop.bid FOR SELECT TO anonymous USING (true);


--
-- Name: config; Type: ROW SECURITY; Schema: shop; Owner: -
--

ALTER TABLE shop.config ENABLE ROW LEVEL SECURITY;

--
-- Name: config config_select_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY config_select_policy ON shop.config FOR SELECT TO seller USING (true);


--
-- Name: product; Type: ROW SECURITY; Schema: shop; Owner: -
--

ALTER TABLE shop.product ENABLE ROW LEVEL SECURITY;

--
-- Name: product product_insert_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY product_insert_policy ON shop.product FOR INSERT TO seller WITH CHECK (((creator_id)::uuid = auth."user"()));


--
-- Name: product product_select_policy; Type: POLICY; Schema: shop; Owner: -
--

CREATE POLICY product_select_policy ON shop.product FOR SELECT TO anonymous USING (true);


--
-- Name: SCHEMA auth; Type: ACL; Schema: -; Owner: -
--

GRANT USAGE ON SCHEMA auth TO anonymous;


--
-- Name: SCHEMA cqrs; Type: ACL; Schema: -; Owner: -
--

GRANT USAGE ON SCHEMA cqrs TO anonymous;


--
-- Name: SCHEMA jobs; Type: ACL; Schema: -; Owner: -
--

GRANT USAGE ON SCHEMA jobs TO admin;


--
-- Name: SCHEMA live; Type: ACL; Schema: -; Owner: -
--

GRANT USAGE ON SCHEMA live TO buyer;


--
-- Name: SCHEMA shop; Type: ACL; Schema: -; Owner: -
--

GRANT USAGE ON SCHEMA shop TO buyer;


--
-- Name: TABLE event; Type: ACL; Schema: cqrs; Owner: -
--

GRANT SELECT,INSERT ON TABLE cqrs.event TO anonymous;


--
-- Name: TABLE config; Type: ACL; Schema: shop; Owner: -
--

GRANT SELECT ON TABLE shop.config TO seller;
GRANT UPDATE ON TABLE shop.config TO admin;


--
-- Name: TABLE person; Type: ACL; Schema: auth; Owner: -
--

GRANT SELECT ON TABLE auth.person TO anonymous;
GRANT INSERT ON TABLE auth.person TO admin;


--
-- Name: TABLE comment; Type: ACL; Schema: live; Owner: -
--

GRANT SELECT ON TABLE live.comment TO anonymous;
GRANT INSERT ON TABLE live.comment TO buyer;


--
-- Name: TABLE show; Type: ACL; Schema: live; Owner: -
--

GRANT SELECT ON TABLE live.show TO anonymous;
GRANT INSERT,UPDATE ON TABLE live.show TO seller;


--
-- Name: TABLE auction; Type: ACL; Schema: shop; Owner: -
--

GRANT SELECT ON TABLE shop.auction TO anonymous;
GRANT INSERT,UPDATE ON TABLE shop.auction TO seller;


--
-- Name: TABLE auction; Type: ACL; Schema: public; Owner: -
--

GRANT SELECT ON TABLE public.auction TO anonymous;


--
-- Name: TABLE bid; Type: ACL; Schema: shop; Owner: -
--

GRANT SELECT ON TABLE shop.bid TO anonymous;
GRANT INSERT ON TABLE shop.bid TO buyer;


--
-- Name: TABLE bid; Type: ACL; Schema: public; Owner: -
--

GRANT SELECT ON TABLE public.bid TO anonymous;


--
-- Name: TABLE comment; Type: ACL; Schema: public; Owner: -
--

GRANT SELECT ON TABLE public.comment TO anonymous;


--
-- Name: TABLE person; Type: ACL; Schema: public; Owner: -
--

GRANT SELECT ON TABLE public.person TO anonymous;


--
-- Name: TABLE product; Type: ACL; Schema: shop; Owner: -
--

GRANT SELECT ON TABLE shop.product TO anonymous;
GRANT INSERT ON TABLE shop.product TO seller;


--
-- Name: TABLE product; Type: ACL; Schema: public; Owner: -
--

GRANT SELECT ON TABLE public.product TO anonymous;


--
-- Name: TABLE show; Type: ACL; Schema: public; Owner: -
--

GRANT SELECT ON TABLE public.show TO anonymous;


--
-- Name: TABLE auction_session; Type: ACL; Schema: shop; Owner: -
--

GRANT SELECT,UPDATE ON TABLE shop.auction_session TO buyer;
GRANT INSERT ON TABLE shop.auction_session TO seller;


--
-- PostgreSQL database dump complete
--

