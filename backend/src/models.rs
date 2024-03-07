use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::schema::*;

#[derive(Serialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGame {
    pub name: String,
}

#[derive(Serialize)]
#[derive(Queryable)]
#[diesel(table_name = game_versions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GameVersion {
    pub game_id: i32,
    pub history_id: i32,
    pub created_date: OffsetDateTime,
    pub deprecated_date: Option<OffsetDateTime>,
}

#[derive(Queryable)]
#[diesel(table_name = game_histories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GameHistory {
    pub id: i32,
}

#[derive(Serialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Store {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStore {
    pub name: String,
}

#[derive(Queryable)]
#[diesel(table_name = store_histories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StoreHistory {
    pub id: i32,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Currency {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCurrency {
    pub name: String,
}

#[derive(Queryable)]
#[diesel(table_name = currency_histories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CurrencyHistory {
    pub id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = purchases)]
#[diesel(belongs_to(Game))]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(Currency))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Purchase {
    pub id: i32,
    pub game_id: i32,
    pub order_id: i32,
    pub currency_id: i32,
    pub amount: bigdecimal::BigDecimal,
}

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = purchases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPurchase {
    pub game_id: i32,
    pub order_id: i32,
    pub currency_id: i32,
    pub amount: bigdecimal::BigDecimal,
}

#[derive(Queryable)]
#[diesel(table_name = purchase_histories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PurchaseHistory {
    pub id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = orders)]
#[diesel(belongs_to(Store))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub id: i32,
    pub order_date: time::Date,
    pub store_id: i32,
}

#[derive(Deserialize)]
#[derive(Insertable)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewOrder {
    pub order_date: time::Date,
    pub store_id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = order_histories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrderHistory {
    pub id: i32,
}
