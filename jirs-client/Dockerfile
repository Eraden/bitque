FROM archlinux:latest

RUN pacman -Sy rustup gcc which --noconfirm

WORKDIR /app/

RUN rustup toolchain install nightly && \
    rustup default nightly && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

ADD ./jirs-data /app/jirs-data

ADD ./jirs-client /app/jirs-client

RUN cd ./jirs-client && \
    rm -Rf build && \
    mkdir build && \
    wasm-pack build --mode normal --release --out-name jirs --out-dir ./build --target web && \
    cp -r ./static/* ./build && \
    cat ./static/index.js \
    | sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" \
    | sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &> ./build/index.js && \
    cp ./js/template.html ./build/index.html && \
    mkdir -p /assets && \
    cp -r ./build/* /assets

CMD cat /app/jirs-client/static/index.js \
    | sed -e "s/process.env.JIRS_SERVER_BIND/'$JIRS_SERVER_BIND'/g" \
    | sed -e "s/process.env.JIRS_SERVER_PORT/'$JIRS_SERVER_PORT'/g" &> /assets/index.js
