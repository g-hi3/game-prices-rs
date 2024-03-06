create table games
(
    id serial primary key,
    name varchar not null
);

-- TODO: Check that there are no overlaps in created date and deprecated date
-- TODO: Check that there is only a single active game_history per game
create table game_history
(
    game_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (game_id, created_date),
    foreign key (game_id) references games (id),
    check (created_date <= current_timestamp),
    check (deprecated_date <= current_timestamp),
    check (created_date < deprecated_date)
);

create view active_games (id, name) as
select id, name
from games
join game_history
        on games.id = game_history.game_id
where deprecated_date is null;
