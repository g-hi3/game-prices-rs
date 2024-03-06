// @generated automatically by Diesel CLI.

diesel::table! {
    currencies (id) {
        id -> Int4,
        #[max_length = 3]
        name -> Bpchar,
    }
}

diesel::table! {
    currency_history (currency_id, created_date) {
        currency_id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    game_history (game_id, created_date) {
        game_id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    games (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    order_history (order_id, created_date) {
        order_id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        purchase_date -> Date,
        store_id -> Int4,
    }
}

diesel::table! {
    purchase_history (purchase_id, created_date) {
        purchase_id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    purchases (id) {
        id -> Int4,
        game_id -> Int4,
        order_id -> Int4,
        currency_id -> Int4,
        amount -> Numeric,
    }
}

diesel::table! {
    store_history (store_id, created_date) {
        store_id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    stores (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(currency_history -> currencies (currency_id));
diesel::joinable!(game_history -> games (game_id));
diesel::joinable!(order_history -> orders (order_id));
diesel::joinable!(orders -> stores (store_id));
diesel::joinable!(purchase_history -> purchases (purchase_id));
diesel::joinable!(purchases -> currencies (currency_id));
diesel::joinable!(purchases -> games (game_id));
diesel::joinable!(purchases -> orders (order_id));
diesel::joinable!(store_history -> stores (store_id));

diesel::allow_tables_to_appear_in_same_query!(
    currencies,
    currency_history,
    game_history,
    games,
    order_history,
    orders,
    purchase_history,
    purchases,
    store_history,
    stores,
);
