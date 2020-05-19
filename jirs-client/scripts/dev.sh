#!/usr/bin/env bash

. .env

rm -Rf tmp
mkdir tmp

wasm-pack build --mode normal --dev --out-name jirs --out-dir ./tmp --target web
../target/debug/jirs-css -i ./js/styles.css -O ./tmp/styles.css

cat ./js/index.js \
| sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" \
| sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &> ./tmp/index.js
cp ./js/template.html ./tmp/index.html

cp -r ./dev/* ./tmp
