use axum::http;
use axum::routing::{delete, get, post};

mod models;
mod schema;
mod services;
mod repositories;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::DELETE])
        .allow_origin("http://localhost:3001".parse::<http::HeaderValue>().unwrap());
    let app = axum::Router::new()
        .route("/games", get(services::get_games))
        .route("/game/create", post(services::create_game))
        .route("/game/update", post(services::update_game))
        .route("/game/:id/delete", delete(services::delete_game))
        .route("/stores", get(services::get_stores))
        .route("/store/create", post(services::create_store))
        .fallback(services::handle_invalid_request)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
