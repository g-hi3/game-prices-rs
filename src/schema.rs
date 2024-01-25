// @generated automatically by Diesel CLI.

diesel::table! {
    currencies (id) {
        id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
        #[max_length = 3]
        name -> Bpchar,
    }
}

diesel::table! {
    games (id) {
        id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
        name -> Varchar,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
        purchase_date -> Date,
        store_id -> Int4,
    }
}

diesel::table! {
    purchases (id) {
        id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
        game_id -> Int4,
        order_id -> Int4,
        currency_id -> Int4,
        amount -> Numeric,
    }
}

diesel::table! {
    stores (id) {
        id -> Int4,
        created_date -> Timestamptz,
        deprecated_date -> Nullable<Timestamptz>,
        name -> Varchar,
    }
}

diesel::joinable!(orders -> stores (store_id));
diesel::joinable!(purchases -> currencies (currency_id));
diesel::joinable!(purchases -> games (game_id));
diesel::joinable!(purchases -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(currencies, games, orders, purchases, stores,);
