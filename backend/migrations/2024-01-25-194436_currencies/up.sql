create table currencies
(
    id serial primary key,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,
    name char(3) not null,

    check (created_date < deprecated_date),
    unique (id, created_date)
);
