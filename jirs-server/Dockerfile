FROM archlinux:latest

WORKDIR /app/

RUN pacman -Sy rustup gcc postgresql --noconfirm

ADD jirs-server .
ADD jirs-data .

RUN rustup toolchain install nightly && \
    rustup default nightly && \
    cargo install diesel_cli --no-default-features --features postgres && \
    cd jirs-server && diesel setup

CMD cd jirs-server && cargo run --bin jirs_server
