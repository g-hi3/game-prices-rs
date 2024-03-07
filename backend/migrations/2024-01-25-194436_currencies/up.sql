create table currencies
(
    id serial primary key,
    name char(3) not null
);

create table currency_versions
(
    currency_id int not null,
    history_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (currency_id, history_id),
    foreign key (currency_id) references currencies (id),
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
        currency_id with =,
        history_id with <>)
);

create view active_currencies (id, name) as
select currencies.id, currencies.name
from histories
join currency_versions
    on histories.id = currency_versions.history_id
join currencies
    on currency_versions.currency_id = currencies.id
where currency_versions.deprecated_date is null;
