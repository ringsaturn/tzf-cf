import { defineWorkersConfig } from "@cloudflare/vitest-pool-workers/config";
import wasmPack from "vite-plugin-wasm-pack";

export default defineWorkersConfig({
  test: {
    poolOptions: {
      workers: {
        wrangler: { configPath: "./wrangler.toml" },
      },
    },
  },
  optimizeDeps: {
    exclude: ["@wasmer/sdk", "tzf-wasm"],
    disabled: true,
  },
  plugins: [wasmPack([], ["tzf-wasm"])],
});
