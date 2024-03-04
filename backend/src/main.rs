use axum::routing::{get, put};
use crate::models::Store;
use crate::schema::stores::dsl::stores;
use diesel::prelude::*;

mod models;
mod schema;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/games", get(services::get_games))
        .route("/game/create", put(services::create_game))
        .route(
            "/stores",
            get(|| async {
                let database_url = std::env::var("DATABASE_URL")
                    .expect("Environment variable DATABASE_URL must be set!");
                let connection = &mut PgConnection::establish(&database_url)
                    .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));
                stores
                    .select(Store::as_select())
                    .load(connection)
                    .expect("Error loading games")
                    .iter()
                    .map(|store| format!("{}: {}", store.id, store.name).to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            }),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
