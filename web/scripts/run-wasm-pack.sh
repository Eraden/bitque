#!/usr/bin/env bash

set -ex

. .env

rm -Rf tmp
mkdir -p tmp
mkdir -p target

cd ${CLIENT_ROOT}
wasm-pack --verbose build --mode ${MODE} ${BUILD_TYPE} --out-name jirs --out-dir ${CLIENT_ROOT}/build --target web

cd ${CLIENT_ROOT}
rm -Rf ${COPY_TO}/styles.css
rsass -t Expanded ${CLIENT_ROOT}/js/styles.css >${COPY_TO}/styles.css

cp -r ${CLIENT_ROOT}/static/* ${COPY_TO}

cat ${CLIENT_ROOT}/static/index.js &>${COPY_TO}/index.js
cp ${CLIENT_ROOT}/build/*.{js,wasm} ${COPY_TO}/
cp ${CLIENT_ROOT}/js/template.html ${COPY_TO}/index.html
