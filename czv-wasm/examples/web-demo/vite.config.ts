import path from "path";
import react from "@vitejs/plugin-react-swc";
import wasmPack from "vite-plugin-wasm-pack";
import { defineConfig } from "vite";

export default defineConfig({
    plugins: [react(), wasmPack([], ["czv-wasm"])],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
});
