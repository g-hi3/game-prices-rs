use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

use crate::models::{Game, NewGame, NewStore, Store};

pub async fn get_games() -> impl IntoResponse {
    let game_data = Game::get_all();
    unwrap_to_response(game_data).await
}

pub async fn create_game(Json(game): Json<NewGame>) -> impl IntoResponse {
    let game_data = Game::create(game);
    unwrap_to_response(game_data).await
}

pub async fn update_game(Json(game): Json<Game>) -> impl IntoResponse {
    let game_data = Game::update(game);
    unwrap_to_response(game_data).await
}

pub async fn delete_game(Path(id): Path<i32>) -> impl IntoResponse {
    match Game::delete(id) {
        Ok(_) => (StatusCode::NO_CONTENT, String::new()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}

pub async fn get_stores() -> impl IntoResponse {
    let store_data = Store::get_all();
    unwrap_to_response(store_data).await
}

pub async fn create_store(Json(store): Json<NewStore>) -> impl IntoResponse {
    let store_data = Store::create(store);
    unwrap_to_response(store_data).await
}

pub async fn update_store(Json(store): Json<Store>) -> impl IntoResponse {
    let store_data = Store::update(store);
    unwrap_to_response(store_data).await
}

pub async fn delete_store(Path(id): Path<i32>) -> impl IntoResponse {
    match Store::delete(id) {
        Ok(_) => (StatusCode::NO_CONTENT, String::new()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}

async fn unwrap_to_response<TOk, TErr>(data: Result<TOk, TErr>) -> impl IntoResponse
    where TOk: Serialize, TErr: std::fmt::Display {
    match data {
        Ok(success_data) => {
            match serde_json::to_string(&success_data) {
                Ok(response_json) => (StatusCode::OK, response_json),
                Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            }
        }
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}

pub async fn handle_invalid_request() -> StatusCode {
    StatusCode::NOT_FOUND
}
