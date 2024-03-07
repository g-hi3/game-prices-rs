create table orders
(
    id serial primary key,
    order_date date not null,
    store_id int not null references stores (id),

    check (order_date <= current_date)
);

create table order_versions
(
    order_id int not null,
    history_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (order_id, history_id),
    foreign key (order_id) references orders (id),
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
        order_id with =,
        history_id with <>)
);

create view active_orders (id, purchase_date, store_id) as
select orders.id, orders.order_date, orders.store_id
from histories
join order_versions
    on histories.id = order_versions.history_id
join orders
    on order_versions.order_id = orders.id
where order_versions.deprecated_date is null;

create table purchases
(
    id serial primary key,
    game_id int not null references games (id),
    order_id int not null references orders (id),
    currency_id int not null references currencies (id),
    amount numeric not null,

    check (amount >= 0)
);

create table purchase_versions
(
    purchase_id int not null,
    history_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (purchase_id, history_id),
    foreign key (purchase_id) references purchases (id),
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
        purchase_id with =,
        history_id with <>)
);

create view active_purchases (id, game_id, order_id, currency_id, amount) as
select purchases.id, purchases.game_id, purchases.order_id, purchases.currency_id, purchases.amount
from histories
join purchase_versions
    on histories.id = purchase_versions.history_id
join purchases
    on purchase_versions.purchase_id = purchases.id
where purchase_versions.deprecated_date is null;
