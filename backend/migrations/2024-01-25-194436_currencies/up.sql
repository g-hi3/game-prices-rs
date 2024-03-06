create table currencies
(
    id serial primary key,
    name char(3) not null
);

create table currency_history
(
    currency_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (currency_id, created_date),
    foreign key (currency_id) references currencies (id),
    check (created_date <= current_timestamp),
    check (deprecated_date <= current_timestamp),
    check (created_date < deprecated_date)
);

create view active_currencies as
select id, name
from currencies
join currency_history
    on id = currency_id
where deprecated_date is null;
