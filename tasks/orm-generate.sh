#!/usr/bin/env bash
set -eu -o pipefail

# https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli

sea-orm-cli generate entity \
    --output-dir=packages/data/src/models \
    --with-serde=serialize \
    --expanded-format \
    --seaography
