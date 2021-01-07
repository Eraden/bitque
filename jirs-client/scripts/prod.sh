#!/usr/bin/env bash

export PROJECT_ROOT=$(git rev-parse --show-toplevel)
export CLIENT_ROOT=${PROJECT_ROOT}/jirs-client
export HI_ROOT=${PROJECT_ROOT}/highlight/jirs-highlight
export MODE=force
export BUILD_TYPE=--release

cd ${PROJECT_ROOT}
cargo build --bin jirs-css

. .env

rm -Rf build
mkdir build

cd $CLIENT_ROOT
wasm-pack build --mode normal --release --out-name jirs --out-dir $CLIENT_ROOT/build --target web

cd $HI_ROOT
wasm-pack build --mode normal --release --out-name hi --out-dir $CLIENT_ROOT/build --target web

${PROJECT_ROOT}/target/debug/jirs-css -i ./js/styles.css -o ./build/styles.css

cp -r ./static/* ./build
cat ./static/index.js \
| sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" \
| sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &> ./build/index.js

cp ./js/template.html ./build/index.html
