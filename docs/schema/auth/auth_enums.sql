--
-- Enums
--

create type auth.role as enum (
  'admin',
  'bidder',
  'seller',
  'viewer'
);
