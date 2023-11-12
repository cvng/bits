# https://github.com/docker/compose

FROM postgres:16

RUN apt-get update
RUN apt-get install --yes postgresql-16-cron

# RUN echo "shared_preload_libraries = 'pg_cron'" >> /var/lib/postgresql/data/postgresql.conf
# RUN echo "cron.database_name = 'postgres'" >> /var/lib/postgresql/data/postgresql.conf

# COPY scripts/db-load-extensions.sh /docker-entrypoint-initdb.d
