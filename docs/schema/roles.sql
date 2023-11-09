--
-- Roles
--

create role admin;
create role bidder;
create role seller;
create role viewer noinherit;

-- Hierarchy

grant viewer to bidder;
grant bidder to seller;
grant seller to admin;

-- Schema

grant usage on schema auth to viewer;
grant usage on schema cqrs to viewer;
grant usage on schema live to bidder;
grant usage on schema shop to bidder;
