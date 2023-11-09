-- Domains

create domain id as uuid;

create domain amount as numeric;

create domain email as text check (value = lower(value) and value like '%@%');
