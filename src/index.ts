/**
 * Welcome to Cloudflare Workers! This is your first worker.
 *
 * - Run `npm run dev` in your terminal to start a development server
 * - Open a browser tab at http://localhost:8787/ to see your worker in action
 * - Run `npm run deploy` to publish your worker
 *
 * Bind resources to your worker in `wrangler.toml`. After adding bindings, a type definition for the
 * `Env` object can be regenerated with `npm run cf-typegen`.
 *
 * Learn more at https://developers.cloudflare.com/workers/
 */

import init, { WasmFinder } from "tzf-wasm";

let finder: WasmFinder;
let initPromise: Promise<void>;

initPromise = (async () => {
  await init();
  finder = new WasmFinder();
  const lng = -74.0060;
  const lat = 40.7128;
  const timezone = finder.get_tz_name(lng, lat);
  console.log(`Timezone for (${lat}, ${lng}): ${timezone}`);
})();

export default {
  async fetch(request, env, ctx): Promise<Response> {
    await initPromise;

    // parse lng, lat from query
    const url = new URL(request.url);
    const lng = parseFloat(url.searchParams.get("lng") || "0");
    const lat = parseFloat(url.searchParams.get("lat") || "0");
    const timezone = finder.get_tz_name(lng, lat);
    return new Response(timezone, { status: 200 });
  },
} satisfies ExportedHandler<Env>;
