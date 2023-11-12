# https://github.com/docker/compose

FROM postgres:16

ARG POSTGRES_DB

RUN apt-get update
RUN apt-get install --yes postgresql-16-cron

RUN echo "shared_preload_libraries = 'pg_cron'" >> /usr/share/postgresql/postgresql.conf.sample
RUN echo "cron.database_name = '${POSTGRES_DB}'" >> /usr/share/postgresql/postgresql.conf.sample
