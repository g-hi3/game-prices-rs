# Entities

## Table of Contents

[Histories](#histories)<br/>
[Games](#games)<br/>
[Stores](#stores)<br/>
[Currencies](#currencies)<br/>
[Orders](#orders)<br/>
[Purchases](#purchases)<br/>

## Histories

The `histories` table contains a single column for identifying a history.
Histories are used by all entities that track version information.
Per version series,
 one record in `histories` will be created.

```mermaid
erDiagram
histories {
    id int PK "not null"
}
```

All tables that use a version series have a table called `<entity>_versions` that stores information for a version of that entity.
Every record of such a versions table has a primary key that consists of the primary key of the associated entity and the primary key of the `histories` table.
Additionally,
 each record contains a `created_date` and an optional `deprecated_date`.

Each version series has one or zero entry where `deprecated_date` is `null`.
The time range between `created_date` and `deprecated_date` may not overlap for records that share the same `history_id`.

A version series can be selected by filtering the versions table by an `history_id` and optionally sorting it by `created_date`.

Some documentation chapters may refer to a version series as a single record,
 because the concept of versions is decoupled from the actual entities.
When an entity documentation mentions "one record per ...",
 it refers to one version series per other entity,
 or multiple records associated with the same history per other entity.

## Games

The `games` table contains data about a specific game.
Games owned in multiple stores are created as a single entity.
Currently,
 a game has only a `name`,
 but in the future it might have more data,
 such as an expected number of hours to complete or number of achievements.

```mermaid
erDiagram
    games {
        id int PK "not null"
        name varchar "not null"
    }
    
    game_versions {
        game_id int PK, FK "not null"
        history_id int PK, FK "not null"
        created_date timestamptz "not null"
        deprecated_date timestamptz "nullable"
    }

    game_versions ||--|| games : "game_id"
    game_versions }|--|| histories : "history_id"
```

## Stores

The `stores` table contains data about a specific store.
Currently,
 the store only has a `name`,
 but in the future there may be added more fields,
 such as how the rating of games in this store should be normalized.

```mermaid
erDiagram
    stores {
        id int PK "not null"
        name varchar "not null"
    }
    
    store_versions {
        store_id int PK, FK "not null"
        history_id int PK, FK "not null"
        created_date timestamptz "not null"
        deprecated_date timestamptz "nullable"
    }

    store_versions ||--|| stores : "store_id"
    store_versions }|--|| histories : "history_id"
```

## Currencies

The `currencies` table contains data about a specific currency that can be used in [orders](#orders).
The currency's name is a three character name,
 that should represent the short name of the currency,
 such as EUR, USD, or JPY.

```mermaid
erDiagram
    currencies {
        id int PK "not null"
        name char(3) "not null"
    }
    
    currency_versions {
        currency_id int PK, FK "not null"
        history_id int PK, FK "not null"
        created_date timestamptz "not null"
        deprecated_date timestamptz "nullable"
    }

    currency_versions ||--|| currencies : "currency_id"
    currency_versions }|--|| histories : "history_id"
```

## Orders

The `orders` table contains data about a specific order from a [store](#stores).
An order is considered a single checkout,
 that may contain one or more [purchased items](#purchases).

```mermaid
erDiagram
    orders {
        id int PK "not null"
        order_date int "not null"
        store_id int FK "not null"
    }
    
    order_versions {
        order_id int PK, FK "not null"
        history_id int PK, FK "not null"
        created_date timestamptz "not null"
        deprecated_date timestamptz "nullable"
    }

    order_versions ||--|| orders : "order_id"
    order_versions }|--|| histories : "history_id"
```


## Purchases

The `purchases` table contains data about a specific purchased item as part of an [order](#orders).
As such,
 purchased items can represent the same [game](#games) in different [stores](#stores).
For games that have been purchased in multiple stores,
 one record will be created per order that was placed containing this game.

```mermaid
erDiagram
    purchases {
        id int PK "not null"
        game_id int FK "not null"
        order_id int FK "not null"
        currency_id int FK "not null"
        amount numeric "not null"
    }
    
    purchase_versions {
        purchase_id int PK, FK "not null"
        history_id int PK, FK "not null"
        created_date timestamptz "not null"
        deprecated_date timestamptz "nullable"
    }

    purchase_versions ||--|| purchases : "purchase_id"
    purchase_versions }|--|| histories : "history_id"
```
