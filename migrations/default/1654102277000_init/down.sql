alter table public.show drop constraint show_pkey;

drop index if exists public.show_pkey;

drop table public.show;

drop domain text_min;
