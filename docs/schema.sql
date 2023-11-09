--

\include_relative schema/schemas.sql
\include_relative schema/domains.sql
\include_relative schema/roles.sql

--

\include_relative schema/auth/person.sql
\include_relative schema/auth/include/auth_utils.sql

\include_relative schema/live/show.sql
\include_relative schema/live/comment.sql

\include_relative schema/shop/product.sql
\include_relative schema/shop/auction.sql
\include_relative schema/shop/bid.sql
\include_relative schema/shop/config.sql

\include_relative schema/cqrs/event.sql
\include_relative schema/cqrs/include/cqrs_utils.sql
\include_relative schema/cqrs/include/cqrs_triggers.sql
\include_relative schema/cqrs/include/cqrs_handlers.sql

--

\include_relative schema/permissions.sql
\include_relative schema/views.sql
