use axum::Json;
use diesel::{Connection, insert_into, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::models::{Game, NewGame};
use crate::schema::games::dsl::games;

pub async fn get_games() -> String {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set!");
    let connection = &mut PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));
    let all_games = games
        .select(Game::as_select())
        .load(connection)
        .expect("Error loading games");
    serde_json::to_string(&all_games).expect("Error serializing games")
}

pub async fn create_game(Json(game): Json<NewGame>) {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set!");
    let connection = &mut PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));
    _ = insert_into(games)
        .values(game)
        .execute(connection)
        .expect("Error inserting game");
}
