import styles from "./App.module.css";

const onToggleHamburger = (pointerEvent) => {
    console.log(pointerEvent);
};

const App = () => {
    return (
        <div class={styles.App}>
            <header class={styles.header}>
                <h1>Game Prices</h1>
            </header>
            <nav class={styles.nav}>
                <button onClick={onToggleHamburger}>|||</button>
                <ul>
                    <li>Games</li>
                    <li>Stores</li>
                    <li>Orders</li>
                </ul>
            </nav>
            <main class={styles.main}>
                <p>Hello World</p>
            </main>
            <footer class={styles.footer}>
                <p>Hi, there are some status infos here.</p>
            </footer>
        </div>
    );
};

export default App;
