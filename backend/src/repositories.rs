use diesel::{Connection, ExpressionMethods, insert_into, PgConnection, QueryDsl, QueryResult, RunQueryDsl, Selectable, SelectableHelper};
use diesel::associations::HasTable;
use diesel::pg::Pg;

use crate::models::{Currency, CurrencyHistory, Game, GameHistory, NewCurrency, NewGame, NewOrder, NewPurchase, NewStore, Order, OrderHistory, Purchase, PurchaseHistory, Store, StoreHistory};
use crate::schema::{currency_versions, game_versions, histories, order_versions, purchase_versions, store_versions};

pub trait HasVersionsTable {
    type VersionsTable: HasTable;

    fn versions_table() -> Self::VersionsTable;
}

// TODO: `Repository` could easily be implemented using a `derive` macro.
pub trait Repository: HasTable + HasVersionsTable {
    type Entity: Selectable<Pg>;
    type HistoryEntity;
    type CreateChangeset;

    fn get_all() -> QueryResult<Vec<Self::Entity>>;
    fn create(changeset: Self::CreateChangeset) -> QueryResult<Self::Entity>;
}

impl HasVersionsTable for Game {
    type VersionsTable = game_versions::table;

    fn versions_table() -> Self::VersionsTable {
        game_versions::table
    }
}

impl Repository for Game {
    type Entity = Game;
    type HistoryEntity = GameHistory;
    type CreateChangeset = NewGame;

    fn get_all() -> QueryResult<Vec<Self::Entity>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(game_versions::deprecated_date.is_null())
            .select(Self::as_select())
            .load(connection)
    }

    fn create(changeset: Self::CreateChangeset) -> QueryResult<Self::Entity> {
        let connection = &mut establish_db_connection();
        let game = connection.transaction(|conn| {
            let history = insert_into(histories::dsl::histories)
                .default_values()
                .get_result::<Self::HistoryEntity>(conn)?;
            let game = insert_into(Self::table())
                .values(changeset)
                .get_result::<Self>(conn)?;
            _ = insert_into(Self::versions_table())
                .values((game_versions::game_id.eq(game.id), game_versions::history_id.eq(history.id)))
                .execute(conn)?;
            Result::<Self, diesel::result::Error>::Ok(game)
        })?;
        Ok(game)
    }
}

impl HasVersionsTable for Store {
    type VersionsTable = store_versions::table;

    fn versions_table() -> Self::VersionsTable {
        store_versions::table
    }
}

impl Repository for Store {
    type Entity = Store;
    type HistoryEntity = StoreHistory;
    type CreateChangeset = NewStore;

    fn get_all() -> QueryResult<Vec<Self::Entity>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(store_versions::deprecated_date.is_null())
            .select(Self::as_select())
            .load(connection)
    }

    fn create(changeset: Self::CreateChangeset) -> QueryResult<Self::Entity> {
        let connection = &mut establish_db_connection();
        let store = connection.transaction(|conn| {
            let history = insert_into(histories::dsl::histories)
                .default_values()
                .get_result::<Self::HistoryEntity>(conn)?;
            let store = insert_into(Self::table())
                .values(changeset)
                .get_result::<Self>(conn)?;
            _ = insert_into(Self::versions_table())
                .values((store_versions::store_id.eq(store.id), store_versions::history_id.eq(history.id)))
                .execute(conn)?;
            Result::<Self, diesel::result::Error>::Ok(store)
        })?;
        Ok(store)
    }
}

impl HasVersionsTable for Currency {
    type VersionsTable = currency_versions::table;

    fn versions_table() -> Self::VersionsTable {
        currency_versions::table
    }
}

impl Repository for Currency {
    type Entity = Currency;
    type HistoryEntity = CurrencyHistory;
    type CreateChangeset = NewCurrency;

    fn get_all() -> QueryResult<Vec<Self::Entity>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(currency_versions::deprecated_date.is_null())
            .select(Self::as_select())
            .load(connection)
    }

    fn create(changeset: Self::CreateChangeset) -> QueryResult<Self::Entity> {
        let connection = &mut establish_db_connection();
        let currency = connection.transaction(|conn| {
            let history = insert_into(histories::dsl::histories)
                .default_values()
                .get_result::<Self::HistoryEntity>(conn)?;
            let currency = insert_into(Self::table())
                .values(changeset)
                .get_result::<Self>(conn)?;
            _ = insert_into(Self::versions_table())
                .values((currency_versions::currency_id.eq(currency.id), currency_versions::history_id.eq(history.id)))
                .execute(conn)?;
            Result::<Self, diesel::result::Error>::Ok(currency)
        })?;
        Ok(currency)
    }
}

impl HasVersionsTable for Order {
    type VersionsTable = order_versions::table;

    fn versions_table() -> Self::VersionsTable {
        order_versions::table
    }
}

impl Repository for Order {
    type Entity = Order;
    type HistoryEntity = OrderHistory;
    type CreateChangeset = NewOrder;

    fn get_all() -> QueryResult<Vec<Self::Entity>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(order_versions::deprecated_date.is_null())
            .select(Self::as_select())
            .load(connection)
    }

    fn create(changeset: Self::CreateChangeset) -> QueryResult<Self::Entity> {
        let connection = &mut establish_db_connection();
        let order = connection.transaction(|conn| {
            let history = insert_into(histories::dsl::histories)
                .default_values()
                .get_result::<Self::HistoryEntity>(conn)?;
            let order = insert_into(Self::table())
                .values(changeset)
                .get_result::<Self>(conn)?;
            _ = insert_into(Self::versions_table())
                .values((order_versions::order_id.eq(order.id), order_versions::history_id.eq(history.id)))
                .execute(conn)?;
            Result::<Self, diesel::result::Error>::Ok(order)
        })?;
        Ok(order)
    }
}

impl HasVersionsTable for Purchase {
    type VersionsTable = purchase_versions::table;

    fn versions_table() -> Self::VersionsTable {
        purchase_versions::table
    }
}

impl Repository for Purchase {
    type Entity = Purchase;
    type HistoryEntity = PurchaseHistory;
    type CreateChangeset = NewPurchase;

    fn get_all() -> QueryResult<Vec<Self::Entity>> {
        let connection = &mut establish_db_connection();
        Self::table()
            .inner_join(Self::versions_table())
            .filter(purchase_versions::deprecated_date.is_null())
            .select(Self::as_select())
            .load(connection)
    }

    fn create(changeset: Self::CreateChangeset) -> QueryResult<Self::Entity> {
        let connection = &mut establish_db_connection();
        let purchase = connection.transaction(|conn| {
            let history = insert_into(histories::dsl::histories)
                .default_values()
                .get_result::<Self::HistoryEntity>(conn)?;
            let purchase = insert_into(Self::table())
                .values(changeset)
                .get_result::<Self>(conn)?;
            _ = insert_into(Self::versions_table())
                .values((purchase_versions::purchase_id.eq(purchase.id), purchase_versions::history_id.eq(history.id)))
                .execute(conn)?;
            Result::<Self, diesel::result::Error>::Ok(purchase)
        })?;
        Ok(purchase)
    }
}

fn establish_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Environment variable DATABASE_URL must be set!");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
