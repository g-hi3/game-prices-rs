create table games
(
    id serial primary key,
    created_date timestamptz not null unique default current_timestamp,
    deprecated_date timestamptz,
    name varchar not null,

    check (created_date < deprecated_date)
);
