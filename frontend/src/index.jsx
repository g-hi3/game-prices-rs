import { render } from "solid-js/web";
import "./index.css";
import App from "./App";

const root = document.getElementById("root");

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
    throw new Error("Root element not found! Please add a <div id=\"root\"> element to the index.html file.");
}

render(() => <App/>, root);
