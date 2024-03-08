use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::models::{Game, NewGame, NewStore, Store};

pub async fn get_games() -> impl IntoResponse {
    match Game::get_all() {
        Ok(all_games) => {
            match serde_json::to_string(&all_games) {
                Ok(response_json) => (StatusCode::OK, response_json),
                Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            }
        }
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}

pub async fn create_game(Json(game): Json<NewGame>) -> impl IntoResponse {
    match Game::create(game) {
        Ok(new_game) => {
            match serde_json::to_string(&new_game) {
                Ok(response_json) => (StatusCode::OK, response_json),
                Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            }
        }
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}

pub async fn update_game(Json(game): Json<Game>) -> impl IntoResponse {
    match Game::update(game) {
        Ok(updated_game) => {
            match serde_json::to_string(&updated_game) {
                Ok(response_json) => (StatusCode::OK, response_json),
                Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            }
        }
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
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
