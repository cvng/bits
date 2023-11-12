-- Prelude

\include_relative roles.sql
\include_relative schemas.sql
\include_relative domains.sql

-- Modules

\include_relative auth/mod.sql
\include_relative live/mod.sql
\include_relative shop/mod.sql
\include_relative cqrs/mod.sql
\include_relative jobs/mod.sql

-- Postlude

\include_relative views.sql
\include_relative permissions.sql
\include_relative policies.sql
