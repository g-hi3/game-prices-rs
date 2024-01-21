use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i32,
    pub name: String,
}
