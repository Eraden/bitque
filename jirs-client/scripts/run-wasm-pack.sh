#!/usr/bin/env bash

set -ex

. .env

rm -Rf tmp
mkdir -p tmp
mkdir -p target

cd ${CLIENT_ROOT}
wasm-pack --verbose build --mode ${MODE} ${BUILD_TYPE} --out-name jirs --out-dir ${CLIENT_ROOT}/build --target web

cd ${CLIENT_ROOT}
${PROJECT_ROOT}/target/debug/jirs-css -i ${CLIENT_ROOT}/js/styles.css -o ${CLIENT_ROOT}/tmp/styles.css

cp -r ${CLIENT_ROOT}/static/* ${CLIENT_ROOT}/tmp

cat ${CLIENT_ROOT}/static/index.js |
  sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" |
  sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &>${CLIENT_ROOT}/tmp/index.js

cp ${CLIENT_ROOT}/build/*.{js,wasm} ${CLIENT_ROOT}/tmp/
cp ${CLIENT_ROOT}/js/template.html ${CLIENT_ROOT}/tmp/index.html
