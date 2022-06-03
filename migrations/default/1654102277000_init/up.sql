create domain text_min as text check(value != '');

create table public.show (
    id uuid not null default gen_random_uuid(),
    name text_min not null
);

alter table public.show enable row level security;

create unique index show_pkey on public.show using btree(id);

alter table public.show add constraint show_pkey primary key using index
show_pkey;
