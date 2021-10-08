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
export CLIENT_ROOT=${PROJECT_ROOT}/web
export HI_ROOT=${PROJECT_ROOT}/highlight/jirs-highlight
export MODE=force
export BUILD_TYPE=--release
export COPY_TO=/tmp/wasm

mkdir -p ${COPY_TO}

echo $PROJECT_ROOT
echo $CLIENT_ROOT

cd ${CLIENT_ROOT}

. .env

${CLIENT_ROOT}/scripts/run-wasm-pack.sh