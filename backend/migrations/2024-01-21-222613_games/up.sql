create extension btree_gist;

create table histories
(
    id serial primary key
);

create table games
(
    id serial primary key,
    name varchar not null
);

create table game_versions
(
    game_id int not null,
    history_id int not null,
    created_date timestamptz not null default current_timestamp,
    deprecated_date timestamptz,

    primary key (game_id, history_id),
    foreign key (game_id) references games (id),
    foreign key (history_id) references histories (id),
    check (created_date <= current_timestamp),
    check (deprecated_date <= current_timestamp),
    check (created_date < deprecated_date),
    exclude using gist (
        history_id with =,
        tstzrange(created_date, deprecated_date, '[]') with &&),
    exclude using gist (history_id with =)
        where (deprecated_date is null),
    exclude using gist (
        game_id with =,
        history_id with <>)
);

create view active_games (id, name) as
select games.id, games.name
from histories
join game_versions
    on histories.id = game_versions.history_id
join games
    on game_versions.game_id = games.id
where game_versions.deprecated_date is null;
