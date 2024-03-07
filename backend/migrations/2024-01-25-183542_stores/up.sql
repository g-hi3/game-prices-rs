create table stores
(
    id serial primary key,
    name varchar not null
);

create table store_versions
(
    store_id int not null,
    history_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (store_id, history_id),
    foreign key (store_id) references stores (id),
    foreign key (history_id) references histories (id),
    check (created_date <= current_timestamp),
    check (deprecated_date <= current_timestamp),
    check (created_date < deprecated_date),
    exclude using gist (
        history_id with =,
        tstzrange(created_date, deprecated_date, '[]') with &&),
    exclude using gist (history_id with =)
        where (deprecated_date is null),
    exclude using gist (
        store_id with =,
        history_id with <>)
);

create view active_stores (id, name) as
select stores.id, stores.name
from histories
join store_versions
    on histories.id = store_versions.history_id
join stores
    on store_versions.store_id = stores.id
where store_versions.deprecated_date is null;
