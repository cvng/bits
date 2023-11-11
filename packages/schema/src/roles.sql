-- Roles

create role viewer;
create role bidder in role viewer;
create role seller in role bidder;
create role admin in role seller;
create role authenticator in role admin login noinherit;
