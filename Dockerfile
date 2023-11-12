# https://github.com/docker/compose

FROM postgres:16

RUN apt-get update
RUN apt-get install --yes postgresql-16-cron

# RUN echo "shared_preload_libraries = 'pg_cron'" >> /var/lib/postgresql/data/postgresql.conf
RUN echo "shared_preload_libraries = 'pg_cron'" >> /usr/share/postgresql/postgresql.conf.sample
RUN echo "cron.database_name = 'bits'" >> /usr/share/postgresql/postgresql.conf.sample
# TODO: PG_DB

COPY scripts/db-load-extensions.sql /docker-entrypoint-initdb.d
