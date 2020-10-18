#!/usr/bin/env bash

PROJECT_ROOT=$(git rev-parse --show-toplevel)
CLIENT_ROOT=${PROJECT_ROOT}/jirs-client

cd ${CLIENT_ROOT}

. .env
cargo watch -s ${CLIENT_ROOT}/scripts/run-wasm-pack.sh -w ${CLIENT_ROOT}/src -w ${CLIENT_ROOT}/Cargo.toml -w ./static -w js
