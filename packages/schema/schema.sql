\include_relative sql/roles.sql
\include_relative sql/schemas.sql
\include_relative sql/domains.sql

\include_relative sql/auth/person.sql
\include_relative sql/auth/include/auth_utils.sql

\include_relative sql/live/show.sql
\include_relative sql/live/comment.sql

\include_relative sql/shop/product.sql
\include_relative sql/shop/auction.sql
\include_relative sql/shop/auction_session.sql
\include_relative sql/shop/bid.sql
\include_relative sql/shop/config.sql
\include_relative sql/shop/include/shop_utils.sql

\include_relative sql/cqrs/event.sql
\include_relative sql/cqrs/include/cqrs_utils.sql
\include_relative sql/cqrs/include/cqrs_triggers.sql
\include_relative sql/cqrs/include/cqrs_handlers.sql

\include_relative sql/permissions.sql
\include_relative sql/policies.sql
\include_relative sql/views.sql
