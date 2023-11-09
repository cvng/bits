-- Util: shop.config()

create function shop.config() returns shop.config
language plpgsql as $$
declare
    config shop.config;
begin
  select * into strict config from shop.config;

  return config;
end; $$;
