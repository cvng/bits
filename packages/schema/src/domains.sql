-- Domains

create domain id as uuid check (value is not null);

create domain amount as numeric check (value >= 0);

create domain email as text check (value = lower(value) and value like '%@%');
