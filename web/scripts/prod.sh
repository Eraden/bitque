#!/usr/bin/env bash

export JIRS_SERVER_BIND=jirs.ita-prog.pl;
export JIRS_SERVER_PORT=443;

RSASS_PATH=$(command -v rsass)
if [[ "${RSASS_PATH}" == "" ]]; then
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
wasm-pack build --mode normal --release --out-name jirs --out-dir ${CLIENT_ROOT}/build --target web
# wasm-pack build --mode normal --dev --out-name jirs --out-dir ${CLIENT_ROOT}/build --target web

#cd $HI_ROOT
#wasm-pack build --mode normal --release --out-name hi --out-dir ${CLIENT_ROOT}/build --target web

cd $CLIENT_ROOT
rm -Rf ${CLIENT_ROOT}/build/styles.css
rsass -t Compressed ${PROJECT_ROOT}/web/js/styles.css > ${CLIENT_ROOT}/build/styles.css

cp -r ${CLIENT_ROOT}/static/* ${CLIENT_ROOT}/build
cat ./static/index.js |
  sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" |
  sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &> ${CLIENT_ROOT}/build/index.js

cp ${CLIENT_ROOT}/js/template.html ${CLIENT_ROOT}/build/index.html
