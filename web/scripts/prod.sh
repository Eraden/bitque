#!/usr/bin/env bash

which rsass
if [[ "$status" != "0" ]]; then
  cargo install rsass --features=commandline
fi

export PROJECT_ROOT=$(git rev-parse --show-toplevel)
export CLIENT_ROOT=${PROJECT_ROOT}/web
export HI_ROOT=${PROJECT_ROOT}/highlight/jirs-highlight
export MODE=force
export BUILD_TYPE=--release

. .env

rm -Rf build
mkdir build

cd $CLIENT_ROOT
wasm-pack build --mode normal --release --out-name jirs --out-dir $CLIENT_ROOT/build --target web

cd $HI_ROOT
wasm-pack build --mode normal --release --out-name hi --out-dir $CLIENT_ROOT/build --target web

cd $CLIENT_ROOT
rm -Rf ${CLIENT_ROOT}/build/styles.css
rsass -t Compressed ${PROJECT_ROOT}/web/js/styles.css > ${CLIENT_ROOT}/build/styles.css

cp -r ./static/* ./build
cat ./static/index.js |
  sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" |
  sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &>./build/index.js

cp ./js/template.html ./build/index.html
