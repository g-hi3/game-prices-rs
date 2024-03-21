import { resolve } from "path";
import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
// import devtools from "solid-devtools/vite";

export default defineConfig({
    plugins: [
        // devtools(),
        solidPlugin()
    ],
    server: {
        port: 3001,
        strictPort: true
    },
    build: {
        target: "esnext",
        rollupOptions: {
            input: {
                main: resolve(__dirname, "index.html")
            }
        }
    }
});