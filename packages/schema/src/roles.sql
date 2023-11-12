-- Roles

create role anonymous with noinherit;

create role authenticated in role anonymous;

create role buyer in role authenticated;

create role seller in role buyer;

create role admin with bypassrls in role seller;

create role authenticator with login in role admin;
