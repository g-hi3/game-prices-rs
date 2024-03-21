import {createResource, For} from "solid-js";

interface Game {
    id: number,
    name: string
}

const fetchGames = async (): Promise<Game[]> => {
    const games = await fetch("http://127.0.0.1:3000/games");
    return games.json();
};

const GameList = () => {
    const [games] = createResource(fetchGames);

    return (
        <>
            <div>Games should be listed here ...</div>
            <For each={games()}>
                {(game, i) => (
                    <div>
                        {i()}: {game.name}
                    </div>
                )}
            </For>
        </>
    );
};

export default GameList;
