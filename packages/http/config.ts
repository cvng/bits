import * as dotenv from "dotenv/mod.ts";

export interface Config {
  graphql: GraphQLConfig;
  server: ServerConfig;
}

export interface GraphQLConfig {
  url: URL;
  secret: string;
}

export interface ServerConfig {
  port: number;
}

export function loadConfig(): Config {
  // https://deno.land/x/dotenv@v3.2.0#safe-mode
  const env = dotenv.config({ safe: true });

  const config: Config = {
    graphql: {
      url: new URL(env["GRAPHQL_URL"]),
      secret: env["HASURA_GRAPHQL_ADMIN_SECRET"],
    },
    server: {
      port: parseInt(env["PORT"]),
    },
  };

  return config;
}
