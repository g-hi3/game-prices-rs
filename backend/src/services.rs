use axum::http::StatusCode;
use axum::Json;
use diesel::{Connection, PgConnection};

use crate::models::{Game, NewGame, NewStore, Store};
use crate::repositories::Repository;

pub async fn get_games() -> String {
    let connection = &mut establish_db_connection();
    let all_games = Game::get_all(connection).expect("Error getting games");
    serde_json::to_string(&all_games).expect("Error serializing games")
}

pub async fn create_game(Json(game): Json<NewGame>) {
    let connection = &mut establish_db_connection();
    _ = Game::create(game, connection);
}

pub async fn get_stores() -> String {
    let connection = &mut establish_db_connection();
    let all_stores = Store::get_all(connection).expect("Error getting stores");
    serde_json::to_string(&all_stores).expect("Error serializing stores")
}

pub async fn create_store(Json(store): Json<NewStore>) {
    let connection = &mut establish_db_connection();
    _ = Store::create(store, connection);
}

fn establish_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set!");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub async fn handle_invalid_request() -> StatusCode {
    StatusCode::NOT_FOUND
}
