import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

// https://github.com/Menci/vite-plugin-wasm
// https://github.com/vitejs/vite/issues/4551
export default defineConfig({
  plugins: [wasm()],
  build: {
    lib: {
      entry: "intermediate/ainu_utils_js.js",
      formats: ["es"],
      fileName: "index",
    },
  },
})
