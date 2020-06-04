FROM archlinux:latest

WORKDIR /app/

RUN pacman -Sy rustup gcc postgresql --noconfirm
RUN rustup toolchain install nightly && \
    rustup default nightly

RUN cargo install diesel_cli --no-default-features --features postgres

ADD jirs-server /app/jirs-server
ADD jirs-data /app/jirs-data

RUN pacman -Sy openssl openssh pkgconf --noconfirm
RUN pkg-config --libs openssl

CMD cd /app/jirs-server && \
    $HOME/.cargo/bin/diesel setup --database-url=$DATABASE_URL && \
    cargo run --bin jirs_server
