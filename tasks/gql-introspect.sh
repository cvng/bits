#!/usr/bin/env bash
set -eu -o pipefail

# https://github.com/hasura/graphqurl

npx graphqurl http://0.0.0.0:8000/graphql --introspect > docs/schema.gql
