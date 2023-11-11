-- Roles

create role anonymous noinherit;

create role buyer in role anonymous;

create role seller in role buyer;

create role admin in role seller;

create role authenticator in role admin login;
