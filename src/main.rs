use crate::models::Game;
use crate::schema::games::dsl::games;
use diesel::prelude::*;

mod models;
mod schema;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello World!" }))
        .route(
            "/games",
            axum::routing::get(|| async {
                dotenvy::dotenv().ok();
                let database_url = std::env::var("DATABASE_URL")
                    .expect("Environment variable DATABASE_URL must be set!");
                let connection = &mut PgConnection::establish(&database_url)
                    .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));
                let results = games
                    .select(Game::as_select())
                    .load(connection)
                    .expect("Error loading games");

                results
                    .iter()
                    .map(|game| format!("{}: {}", game.id, game.name).to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            }),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
