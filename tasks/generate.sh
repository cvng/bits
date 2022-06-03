set -e

export $(cat .env | xargs)

npx graphqurl "$GRAPHQL_URL" \
    -H "x-hasura-admin-secret: $HASURA_GRAPHQL_ADMIN_SECRET" \
    --introspect \
    > docs/schema.gql

npx graphql-codegen --config .graphqlrc.yaml

# Patch import in generated files (as of now, only "gqlImport" is configurable).
# https://www.graphql-code-generator.com/plugins/typescript-graphql-request
sed -i '' '/graphql-request/d' packages/sdk/sdk.ts
sed -i '' '/graphql-tag/d' packages/sdk/sdk.ts
sed -i '' '1 i \
import { gql, GraphQLClient } from "graphql_request/mod.ts"; import * as Dom from "graphql_request/src/types.dom.ts";
' packages/sdk/sdk.ts

deno fmt
