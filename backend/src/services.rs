use axum::Json;
use diesel::{Connection, ExpressionMethods, insert_into, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use time::OffsetDateTime;

use crate::models::{Game, NewGame};
use crate::schema::game_history::{created_date, deprecated_date, game_id};
use crate::schema::game_history::dsl::game_history;
use crate::schema::games::dsl::games;

pub async fn get_games() -> String {
    let connection = &mut establish_db_connection();
    let all_valid_games = games
        .left_join(game_history)
        .filter(deprecated_date.is_null())
        .select(Game::as_select())
        .load(connection)
        .expect("Error querying games");
    serde_json::to_string(&all_valid_games).expect("Error serializing games")
}

pub async fn create_game(Json(game): Json<NewGame>) {
    let now = OffsetDateTime::now_utc();
    let connection = &mut establish_db_connection();
    connection
        .transaction(|conn| {
            let inserted_game = insert_into(games)
                .values(game)
                .get_result::<Game>(conn)?;
            _ = insert_into(game_history)
                .values((game_id.eq(inserted_game.id), created_date.eq(now)))
                .execute(conn)?;
            diesel::QueryResult::Ok(())
        })
        .expect("Error creating game");
}

fn establish_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set!");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
