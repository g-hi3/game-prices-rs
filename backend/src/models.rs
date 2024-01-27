use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i32,
    pub created_date: time::OffsetDateTime,
    pub deprecated_date: Option<time::OffsetDateTime>,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = stores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Store {
    pub id: i32,
    pub created_date: time::OffsetDateTime,
    pub deprecated_date: Option<time::OffsetDateTime>,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Currency {
    pub id: i32,
    pub created_date: time::OffsetDateTime,
    pub deprecated_date: Option<time::OffsetDateTime>,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = purchases)]
#[diesel(belongs_to(Game))]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(Currency))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Purchase {
    pub id: i32,
    pub created_date: time::OffsetDateTime,
    pub deprecated_date: Option<time::OffsetDateTime>,
    pub game_id: i32,
    pub order_id: i32,
    pub currency_id: i32,
    pub amount: bigdecimal::BigDecimal,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = orders)]
#[diesel(belongs_to(Store))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub id: i32,
    pub created_date: time::OffsetDateTime,
    pub deprecated_date: Option<time::OffsetDateTime>,
    pub purchase_date: time::Date,
    pub store_id: i32,
}
