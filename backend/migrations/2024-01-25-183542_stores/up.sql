create table stores
(
    id serial primary key,
    name varchar not null
);

create table store_history
(
    store_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (store_id, created_date),
    foreign key (store_id) references stores (id),
    check (created_date <= current_timestamp),
    check (deprecated_date <= current_timestamp),
    check (created_date < deprecated_date)
);

create view active_stores as
select id, name
from stores
join store_history
    on id = store_id
where deprecated_date is null;
