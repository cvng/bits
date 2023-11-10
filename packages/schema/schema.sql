\include_relative src/roles.sql
\include_relative src/schemas.sql
\include_relative src/domains.sql

\include_relative src/auth/person.sql
\include_relative src/auth/include/auth_utils.sql

\include_relative src/live/show.sql
\include_relative src/live/comment.sql

\include_relative src/shop/product.sql
\include_relative src/shop/auction.sql
\include_relative src/shop/auction_session.sql
\include_relative src/shop/bid.sql
\include_relative src/shop/config.sql
\include_relative src/shop/include/shop_utils.sql

\include_relative src/cqrs/event.sql
\include_relative src/cqrs/include/cqrs_utils.sql
\include_relative src/cqrs/include/cqrs_triggers.sql
\include_relative src/cqrs/include/cqrs_handlers.sql

\include_relative src/permissions.sql
\include_relative src/policies.sql
\include_relative src/views.sql
