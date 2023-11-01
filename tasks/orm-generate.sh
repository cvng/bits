#!/bin/zsh
set -o errexit -o nounset -o pipefail

# https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli

sea-orm-cli generate entity \
    --output-dir=packages/data/src/entities \
    --with-serde=serialize \
    --expanded-format \
    --seaography
