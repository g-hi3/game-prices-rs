create table orders
(
    id serial primary key,
    purchase_date date not null,
    store_id int not null references stores (id),

    check (purchase_date <= current_date)
);

create table order_history
(
    order_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (order_id, created_date),
    foreign key (order_id) references orders (id),
    check (created_date <= current_timestamp),
    check (deprecated_date <= current_timestamp),
    check (created_date < deprecated_date)
);

create view active_orders as
select id, purchase_date, store_id
from orders
join order_history
    on id = order_id
where deprecated_date is null;

create table purchases
(
    id serial primary key,
    game_id int not null references games (id),
    order_id int not null references orders (id),
    currency_id int not null references currencies (id),
    amount numeric not null,

    check (amount >= 0)
);

create table purchase_history
(
    purchase_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (purchase_id, created_date),
    foreign key (purchase_id) references purchases (id),
    check (created_date <= current_timestamp),
    check (deprecated_date <= current_timestamp),
    check (created_date < deprecated_date)
);

create view active_purchases as
select id, game_id, order_id, currency_id, amount
from purchases
join purchase_history
    on id = purchase_id
where deprecated_date is null;
