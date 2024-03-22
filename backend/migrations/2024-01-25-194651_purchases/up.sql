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
    created_date timestamptz not null default clock_timestamp(),
    deprecated_date timestamptz,

    primary key (order_id, history_id),
    foreign key (order_id) references orders (id),
    foreign key (history_id) references histories (id),
    check (created_date <= clock_timestamp()),
    check (deprecated_date <= clock_timestamp()),
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

create table game_purchases
(
    id serial primary key,
    game_id int not null references games (id),
    order_id int not null references orders (id),
    currency_id int not null references currencies (id),
    amount numeric not null,

    check (amount >= 0)
);

create table game_purchase_versions
(
    purchase_id int not null,
    history_id int not null,
    created_date timestamptz not null default clock_timestamp(),
    deprecated_date timestamptz,

    primary key (purchase_id, history_id),
    foreign key (purchase_id) references game_purchases (id),
    foreign key (history_id) references histories (id),
    check (created_date <= clock_timestamp()),
    check (deprecated_date <= clock_timestamp()),
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

create view active_game_purchases (id, game_id, order_id, currency_id, amount) as
select game_purchases.id, game_purchases.game_id, game_purchases.order_id, game_purchases.currency_id, game_purchases.amount
from histories
join game_purchase_versions
    on histories.id = game_purchase_versions.history_id
join game_purchases
    on game_purchase_versions.purchase_id = game_purchases.id
where game_purchase_versions.deprecated_date is null;
