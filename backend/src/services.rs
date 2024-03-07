use axum::http::StatusCode;
use axum::Json;

use crate::models::{Game, NewGame, NewStore, Store};
use crate::repositories::Repository;

pub async fn get_games() -> String {
    let all_games = Game::get_all().expect("Error getting games");
    serde_json::to_string(&all_games).expect("Error serializing games")
}

pub async fn create_game(Json(game): Json<NewGame>) {
    _ = Game::create(game);
}

pub async fn get_stores() -> String {
    let all_stores = Store::get_all().expect("Error getting stores");
    serde_json::to_string(&all_stores).expect("Error serializing stores")
}

pub async fn create_store(Json(store): Json<NewStore>) {
    _ = Store::create(store);
}

pub async fn handle_invalid_request() -> StatusCode {
    StatusCode::NOT_FOUND
}
