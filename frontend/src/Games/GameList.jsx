import {createResource, For} from "solid-js";

const fetchGames = async () => {
    const games = await fetch("http://127.0.0.1:3000/games");
    return games.json();
};

const fetchStores = async () => {
    const stores = await fetch("http://127.0.0.1:3000/stores");
    return stores.json();
}

const GameList = () => {
    const [games] = createResource(fetchGames);
    const [stores] = createResource(fetchStores);

    return (
        <>
            <div>Games should be listed here ...</div>
            <For each={games()}>
                {(game, i) => (
                    <div>
                        {i}: {game.name}
                    </div>
                )}
            </For>
        </>
    );
};

export default GameList;
