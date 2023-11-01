#!/bin/zsh
set -o errexit -o nounset -o pipefail

# https://github.com/hasura/graphqurl

npx graphqurl http://0.0.0.0:8000/graphql --introspect > docs/schema.gql
