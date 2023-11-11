-- Util: auth.login()

create function auth.login(user_id id) returns auth.role
language plpgsql as $$
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

-- Util: auth.role()

create function auth.role() returns auth.role
language plpgsql as $$
begin
  return current_setting('role')::auth.role;
end; $$;

-- Util: auth.user()

create function auth.user() returns id
language plpgsql as $$
begin
  return current_setting('auth.user')::id;
end; $$;

-- Util: auth.logout()

create function auth.logout() returns void
language plpgsql as $$
begin
  reset "role";
  reset "auth.user";
end; $$;
