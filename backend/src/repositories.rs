use diesel::{Connection, ExpressionMethods, NullableExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;
use diesel::dsl::IsNull;
use time::OffsetDateTime;

use crate::models::{Currency, CurrencyVersion, Game, GamePurchase, GamePurchaseVersion, GameVersion, History, NewCurrency, NewGame, NewGamePurchase, NewGameVersion, NewOrder, NewStore, NewStoreVersion, Order, OrderVersion, Store, StoreVersion};
use crate::schema::{currency_versions, game_purchase_versions, game_versions, histories, order_versions, store_versions};

fn create_history_no_transaction(connection: &mut PgConnection) -> QueryResult<History> {
    diesel::insert_into(histories::dsl::histories)
        .default_values()
        .get_result::<History>(connection)
}

pub trait HasVersionsTable {
    type VersionsTable: HasTable;
    type ActiveState: NullableExpressionMethods;

    fn versions_table() -> Self::VersionsTable;
    fn only_active() -> IsNull<Self::ActiveState>;
}

impl HasVersionsTable for Game {
    type VersionsTable = game_versions::table;
    type ActiveState = game_versions::deprecated_date;

    fn versions_table() -> Self::VersionsTable {
        game_versions::table
    }

    fn only_active() -> IsNull<Self::ActiveState> {
        game_versions::deprecated_date.is_null()
    }
}

impl Game {
    pub fn get_all() -> QueryResult<Vec<Self>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(Self::only_active())
            .select(Self::as_select())
            .load(connection)
    }

    pub fn create(changeset: NewGame) -> QueryResult<Self> {
        let connection = &mut establish_db_connection();
        let game = connection.transaction(|conn| {
            let history = create_history_no_transaction(conn)?;
            let game = Self::create_no_transaction(changeset, conn)?;
            let game_version = NewGameVersion {
                game_id: game.id,
                history_id: history.id,
            };
            _ = GameVersion::create_no_transaction(game_version, conn)?;
            Result::<Self, diesel::result::Error>::Ok(game)
        })?;
        Ok(game)
    }

    pub fn update(changeset: Self) -> QueryResult<Self> {
        let connection = &mut establish_db_connection();
        let game = connection.transaction(|conn| {
            let next_game = NewGame {
                name: changeset.name
            };
            let next_game = Self::create_no_transaction(next_game, conn)?;
            let mut prev_version = GameVersion::get_by_id(changeset.id, conn)?;
            let next_version = NewGameVersion {
                game_id: next_game.id,
                history_id: prev_version.history_id,
            };
            prev_version.deprecated_date = Some(OffsetDateTime::now_utc());
            _ = GameVersion::update_no_transaction(prev_version, conn)?;
            _ = GameVersion::create_no_transaction(next_version, conn)?;
            Result::<Self, diesel::result::Error>::Ok(next_game)
        })?;
        Ok(game)
    }

    pub fn delete(id: i32) -> QueryResult<()> {
        let connection = &mut establish_db_connection();
        connection.transaction(|conn| {
            let mut current_version = GameVersion::get_by_id(id, conn)?;
            current_version.deprecated_date = Some(OffsetDateTime::now_utc());
            _ = GameVersion::update_no_transaction(current_version, conn)?;
            Ok(())
        })
    }

    fn create_no_transaction(changeset: NewGame, connection: &mut PgConnection) -> QueryResult<Self> {
        diesel::insert_into(Self::table())
            .values(changeset)
            .get_result::<Self>(connection)
    }
}

impl GameVersion {
    fn get_by_id(id: i32, connection: &mut PgConnection) -> QueryResult<Self> {
        Self::table()
            .filter(game_versions::game_id.eq(id))
            .filter(<Game as HasVersionsTable>::only_active())
            .get_result::<Self>(connection)
    }

    fn create_no_transaction(changeset: NewGameVersion, connection: &mut PgConnection) -> QueryResult<Self> {
        diesel::insert_into(Self::table())
            .values(changeset)
            .get_result::<Self>(connection)
    }

    fn update_no_transaction(changeset: Self, connection: &mut PgConnection) -> QueryResult<Self> {
        diesel::update(Self::table())
            .filter(game_versions::game_id.eq(changeset.game_id))
            .set(changeset)
            .get_result::<Self>(connection)
    }
}

impl HasVersionsTable for Store {
    type VersionsTable = store_versions::table;
    type ActiveState = store_versions::deprecated_date;

    fn versions_table() -> Self::VersionsTable {
        store_versions::table
    }

    fn only_active() -> IsNull<Self::ActiveState> {
        store_versions::deprecated_date.is_null()
    }
}

impl Store {
    pub fn get_all() -> QueryResult<Vec<Self>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(Self::only_active())
            .select(Self::as_select())
            .load(connection)
    }

    pub fn create(changeset: NewStore) -> QueryResult<Self> {
        let connection = &mut establish_db_connection();
        let store = connection.transaction(|conn| {
            let history = create_history_no_transaction(conn)?;
            let store = Self::create_no_transaction(changeset, conn)?;
            let store_version = NewStoreVersion {
                store_id: store.id,
                history_id: history.id,
            };
            _ = StoreVersion::create_no_transaction(store_version, conn)?;
            Result::<Self, diesel::result::Error>::Ok(store)
        })?;
        Ok(store)
    }

    pub fn update(changeset: Self) -> QueryResult<Self> {
        let connection = &mut establish_db_connection();
        let store = connection.transaction(|conn| {
            let next_store = NewStore {
                name: changeset.name
            };
            let next_store = Self::create_no_transaction(next_store, conn)?;
            let mut prev_version = StoreVersion::get_by_id(changeset.id, conn)?;
            let next_version = NewStoreVersion {
                store_id: next_store.id,
                history_id: prev_version.history_id,
            };
            prev_version.deprecated_date = Some(OffsetDateTime::now_utc());
            _ = StoreVersion::update_no_transaction(prev_version, conn)?;
            _ = StoreVersion::create_no_transaction(next_version, conn)?;
            Result::<Self, diesel::result::Error>::Ok(next_store)
        })?;
        Ok(store)
    }

    pub fn delete(id: i32) -> QueryResult<()> {
        let connection = &mut establish_db_connection();
        connection.transaction(|conn| {
            let mut current_version = StoreVersion::get_by_id(id, conn)?;
            current_version.deprecated_date = Some(OffsetDateTime::now_utc());
            _ = StoreVersion::update_no_transaction(current_version, conn)?;
            Ok(())
        })
    }

    fn create_no_transaction(changeset: NewStore, connection: &mut PgConnection) -> QueryResult<Self> {
        diesel::insert_into(Self::table())
            .values(changeset)
            .get_result::<Self>(connection)
    }
}

impl HasTable for GameVersion {
    type Table = game_versions::table;

    fn table() -> Self::Table {
        game_versions::table
    }
}

impl HasTable for StoreVersion {
    type Table = store_versions::table;

    fn table() -> Self::Table {
        store_versions::table
    }
}

impl StoreVersion {
    fn get_by_id(id: i32, connection: &mut PgConnection) -> QueryResult<Self> {
        Self::table()
            .filter(store_versions::store_id.eq(id))
            .filter(<Store as HasVersionsTable>::only_active())
            .get_result::<Self>(connection)
    }

    fn create_no_transaction(changeset: NewStoreVersion, connection: &mut PgConnection) -> QueryResult<Self> {
        diesel::insert_into(Self::table())
            .values(changeset)
            .get_result::<Self>(connection)
    }

    fn update_no_transaction(changeset: Self, connection: &mut PgConnection) -> QueryResult<Self> {
        diesel::update(Self::table())
            .filter(store_versions::store_id.eq(changeset.store_id))
            .set(changeset)
            .get_result::<Self>(connection)
    }
}

impl HasVersionsTable for Currency {
    type VersionsTable = currency_versions::table;
    type ActiveState = currency_versions::deprecated_date;

    fn versions_table() -> Self::VersionsTable {
        currency_versions::table
    }

    fn only_active() -> IsNull<Self::ActiveState> {
        currency_versions::deprecated_date.is_null()
    }
}

impl Currency {
    pub fn get_all() -> QueryResult<Vec<Self>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(Self::only_active())
            .select(Self::as_select())
            .load(connection)
    }

    pub fn create(changeset: NewCurrency) -> QueryResult<Self> {
        let connection = &mut establish_db_connection();
        let currency = connection.transaction(|conn| {
            let history = diesel::insert_into(histories::dsl::histories)
                .default_values()
                .get_result::<History>(conn)?;
            let currency = diesel::insert_into(Self::table())
                .values(changeset)
                .get_result::<Self>(conn)?;
            _ = diesel::insert_into(Self::versions_table())
                .values((currency_versions::currency_id.eq(currency.id), currency_versions::history_id.eq(history.id)))
                .execute(conn)?;
            Result::<Self, diesel::result::Error>::Ok(currency)
        })?;
        Ok(currency)
    }
}

impl HasTable for CurrencyVersion {
    type Table = currency_versions::table;

    fn table() -> Self::Table {
        currency_versions::table
    }
}

impl HasVersionsTable for Order {
    type VersionsTable = order_versions::table;
    type ActiveState = order_versions::deprecated_date;

    fn versions_table() -> Self::VersionsTable {
        order_versions::table
    }

    fn only_active() -> IsNull<Self::ActiveState> {
        order_versions::deprecated_date.is_null()
    }
}

impl Order {
    pub fn get_all() -> QueryResult<Vec<Self>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(Self::only_active())
            .select(Self::as_select())
            .load(connection)
    }

    fn create(changeset: NewOrder) -> QueryResult<Self> {
        let connection = &mut establish_db_connection();
        let order = connection.transaction(|conn| {
            let history = diesel::insert_into(histories::dsl::histories)
                .default_values()
                .get_result::<History>(conn)?;
            let order = diesel::insert_into(Self::table())
                .values(changeset)
                .get_result::<Self>(conn)?;
            _ = diesel::insert_into(Self::versions_table())
                .values((order_versions::order_id.eq(order.id), order_versions::history_id.eq(history.id)))
                .execute(conn)?;
            Result::<Self, diesel::result::Error>::Ok(order)
        })?;
        Ok(order)
    }
}

impl HasTable for OrderVersion {
    type Table = order_versions::table;

    fn table() -> Self::Table {
        order_versions::table
    }
}

impl HasVersionsTable for GamePurchase {
    type VersionsTable = game_purchase_versions::table;
    type ActiveState = game_purchase_versions::deprecated_date;

    fn versions_table() -> Self::VersionsTable {
        game_purchase_versions::table
    }

    fn only_active() -> IsNull<Self::ActiveState> {
        game_purchase_versions::deprecated_date.is_null()
    }
}

impl GamePurchase {
    fn get_all() -> QueryResult<Vec<Self>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(Self::only_active())
            .select(Self::as_select())
            .load(connection)
    }

    fn create(changeset: NewGamePurchase) -> QueryResult<Self> {
        let connection = &mut establish_db_connection();
        let purchase = connection.transaction(|conn| {
            let history = diesel::insert_into(histories::dsl::histories)
                .default_values()
                .get_result::<History>(conn)?;
            let purchase = diesel::insert_into(Self::table())
                .values(changeset)
                .get_result::<Self>(conn)?;
            _ = diesel::insert_into(Self::versions_table())
                .values((game_purchase_versions::purchase_id.eq(purchase.id), game_purchase_versions::history_id.eq(history.id)))
                .execute(conn)?;
            Result::<Self, diesel::result::Error>::Ok(purchase)
        })?;
        Ok(purchase)
    }
}

impl HasTable for GamePurchaseVersion {
    type Table = game_purchase_versions::table;

    fn table() -> Self::Table {
        game_purchase_versions::table
    }
}

fn establish_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set!");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
