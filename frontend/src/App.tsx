import styles from "./App.module.css";
import FallbackComponent from "./FallbackComponent.tsx";
import GameList from "./Games/GameList.tsx";
import StoreList from "./Stores/StoreList.tsx";
import OrderList from "./Orders/OrderList.tsx";
import {createSignal} from "solid-js";
import {Dynamic} from "solid-js/web";

const App = () => {
    const [activeComponent, setActiveComponent] = createSignal(FallbackComponent, {
        name: "activeComponent",
        internal: true
    });

    const onToggleHamburger = (pointerEvent: MouseEvent) => {
        console.log(pointerEvent);
    };

    return (
        <div class={styles.App}>
            <header class={styles.header}>
                <h1>Game Prices</h1>
                <button onClick={onToggleHamburger}>|||</button>
            </header>
            <nav class={styles.nav}>
                <ul>
                    <li onClick={() => setActiveComponent(() => GameList)}>Games</li>
                    <li onClick={() => setActiveComponent(() => StoreList)}>Stores</li>
                    <li onClick={() => setActiveComponent(() => OrderList)}>Orders</li>
                </ul>
            </nav>
            <main class={styles.main}>
                <Dynamic component={activeComponent()}/>
            </main>
            <footer class={styles.footer}>
                <p>Hi, there are some status infos here.</p>
            </footer>
        </div>
    );
};

export default App;
