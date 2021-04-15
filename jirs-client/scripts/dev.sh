#!/usr/bin/env bash

RSASS_PATH=$(command -v rsass)
if [[ "${RSASS_PATH}" == "" ]]; then
  cargo install rsass --features=commandline
fi

WASM_PACK_PATH=$(command -v wasm-pack)
if [[ "${WASM_PACK_PATH}" == "" ]]; then
  cargo install wasm-pack
fi

export PROJECT_ROOT=$(git rev-parse --show-toplevel)
export CLIENT_ROOT=${PROJECT_ROOT}/jirs-client
export HI_ROOT=${PROJECT_ROOT}/highlight/jirs-highlight
export MODE=force
export BUILD_TYPE=--dev

cd ${CLIENT_ROOT}

. .env

cargo watch \
  -i ./jirs-client/src/location.rs \
  -s ${CLIENT_ROOT}/scripts/run-wasm-pack.sh \
  -w ${CLIENT_ROOT}/src \
  -w ${CLIENT_ROOT}/Cargo.toml \
  -w ./static \
  -w js
