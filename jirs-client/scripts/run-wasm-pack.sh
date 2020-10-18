#!/usr/bin/env bash

set -ex

. .env

rm -Rf tmp
mkdir -p tmp
mkdir -p target

wasm-pack build --mode force --dev --out-name jirs --out-dir ./tmp --target web -- --verbose

../target/debug/jirs-css -i ./js/styles.css -O ./tmp/styles.css

cp -r ./static/* ./tmp
cat ./static/index.js |
  sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" |
  sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &>./tmp/index.js

cp ./js/template.html ./tmp/index.html
