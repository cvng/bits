import { serve } from "std/http/server.ts";
import { loadConfig } from "./config.ts";
import { createHandler } from "./handler.ts";

async function main() {
  const config = await loadConfig();

  const handler = createHandler(config);

  serve(handler, { port: config.server.port });
}

if (import.meta.main) {
  main();
}
