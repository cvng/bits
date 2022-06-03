import { GraphQLConfig } from "bits/http/config.ts";
import { GraphQLClient } from "graphql_request/mod.ts";
import { getSdk, Sdk } from "./sdk.ts";

export * from "./sdk.ts";

export function initSdk(config: GraphQLConfig): Sdk {
  const client = new GraphQLClient(config.url.toString(), {
    headers: {
      "x-hasura-admin-secret": config.secret,
    },
  });

  return getSdk(client);
}
