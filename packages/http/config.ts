import * as dotenv from "std/dotenv/mod.ts";

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

export async function loadConfig(): Promise<Config> {
  // https://deno.land/std@0.144.0/dotenv#safe-mode
  const env = await dotenv.config({ safe: true });

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
