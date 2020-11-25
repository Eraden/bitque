#!/usr/bin/env bash

export PROJECT_ROOT=$(git rev-parse --show-toplevel)
export CLIENT_ROOT=${PROJECT_ROOT}/jirs-client
export HI_ROOT=${PROJECT_ROOT}/highlight/jirs-highlight
export MODE=force
export BUILD_TYPE=--dev

cd ${CLIENT_ROOT}

. .env

cargo watch \
  -s ${CLIENT_ROOT}/scripts/run-wasm-pack.sh \
  -w ${CLIENT_ROOT}/src \
  -w ${CLIENT_ROOT}/Cargo.toml \
  -w ./static \
  -w js
