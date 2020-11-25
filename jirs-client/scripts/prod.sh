#!/usr/bin/env bash

. .env

rm -Rf build
mkdir build

cd $CLIENT_ROOT
wasm-pack build --mode normal --release --out-name jirs --out-dir $CLIENT_ROOT/build --target web

cd $HI_ROOT
wasm-pack build --mode normal --release --out-name hi --out-dir $CLIENT_ROOT/build --target web

cargo run --bin jirs-css -i ./js/styles.css -o ./build/styles.css

cp -r ./static/* ./build
cat ./static/index.js \
| sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" \
| sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &> ./build/index.js

cp ./js/template.html ./build/index.html
