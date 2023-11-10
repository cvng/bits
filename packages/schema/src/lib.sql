\include_relative roles.sql
\include_relative schemas.sql
\include_relative domains.sql

\include_relative auth/person.sql
\include_relative auth/include/auth_utils.sql

\include_relative live/show.sql
\include_relative live/comment.sql

\include_relative shop/product.sql
\include_relative shop/auction.sql
\include_relative shop/auction_session.sql
\include_relative shop/bid.sql
\include_relative shop/config.sql
\include_relative shop/include/shop_utils.sql

\include_relative cqrs/event.sql
\include_relative cqrs/include/cqrs_utils.sql
\include_relative cqrs/include/cqrs_triggers.sql
\include_relative cqrs/include/cqrs_handlers.sql

\include_relative permissions.sql
\include_relative policies.sql
\include_relative views.sql
