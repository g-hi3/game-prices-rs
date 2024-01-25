create table orders
(
    id serial primary key,
    created_date timestamptz not null unique default current_timestamp,
    deprecated_date timestamptz,
    purchase_date date not null,
    store_id int not null references stores(id),

    check (created_date < deprecated_date)
);

create table purchases
(
    id serial primary key,
    created_date timestamptz not null unique default current_timestamp,
    deprecated_date timestamptz,
    game_id int not null references games(id),
    order_id int not null references orders(id),
    currency_id int not null references currencies(id),
    amount numeric not null,

    check (created_date < deprecated_date),
    check (amount >= 0)
);
