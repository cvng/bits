#!/usr/bin/env bash
set -eu -o pipefail

cargo --version # > cargo 1.75.0-nightly (8eb8acbb1 2023-10-17)
rustc --version # > rustc 1.75.0-nightly (0039d739d 2023-10-18)
docker --version # > Docker version 24.0.6, build ed223bc
python3 --version # > Python 3.11.5
psql --version # > psql (PostgreSQL) 16.1 (Homebrew)
