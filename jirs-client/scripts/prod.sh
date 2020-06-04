#!/usr/bin/env bash

. .env

rm -Rf build
mkdir build

wasm-pack build --mode normal --release --out-name jirs --out-dir ./build --target web
../target/debug/jirs-css -i ./js/styles.css -O ./build/styles.css

cp -r ./static/* ./build
cat ./static/index.js \
| sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" \
| sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &> ./build/index.js

cp ./js/template.html ./build/index.html
