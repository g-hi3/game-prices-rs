// @generated automatically by Diesel CLI.

diesel::table! {
    currencies (id) {
        id -> Int4,
        #[max_length = 3]
        name -> Bpchar,
    }
}

diesel::table! {
    currency_versions (currency_id, history_id) {
        currency_id -> Int4,
        history_id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    game_versions (game_id, history_id) {
        game_id -> Int4,
        history_id -> Int4,
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
    histories (id) {
        id -> Int4,
    }
}

diesel::table! {
    order_histories (id) {
        id -> Int4,
    }
}

diesel::table! {
    order_versions (order_id, history_id) {
        order_id -> Int4,
        history_id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        order_date -> Date,
        store_id -> Int4,
    }
}

diesel::table! {
    purchase_versions (purchase_id, history_id) {
        purchase_id -> Int4,
        history_id -> Int4,
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
    store_versions (store_id, history_id) {
        store_id -> Int4,
        history_id -> Int4,
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

diesel::joinable!(currency_versions -> currencies (currency_id));
diesel::joinable!(currency_versions -> histories (history_id));
diesel::joinable!(game_versions -> games (game_id));
diesel::joinable!(game_versions -> histories (history_id));
diesel::joinable!(order_versions -> histories (history_id));
diesel::joinable!(order_versions -> orders (order_id));
diesel::joinable!(orders -> stores (store_id));
diesel::joinable!(purchase_versions -> histories (history_id));
diesel::joinable!(purchase_versions -> purchases (purchase_id));
diesel::joinable!(purchases -> currencies (currency_id));
diesel::joinable!(purchases -> games (game_id));
diesel::joinable!(purchases -> orders (order_id));
diesel::joinable!(store_versions -> histories (history_id));
diesel::joinable!(store_versions -> stores (store_id));

diesel::allow_tables_to_appear_in_same_query!(
    currencies,
    currency_versions,
    game_versions,
    games,
    histories,
    order_histories,
    order_versions,
    orders,
    purchase_versions,
    purchases,
    store_versions,
    stores,
);
