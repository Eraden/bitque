# A simplified Jira clone built with seed.rs and actix

[![builds.sr.ht status](https://builds.sr.ht/~tsumanu/jirs.sr.ht.svg)](https://builds.sr.ht/~tsumanu/jirs.sr.ht?)

https://git.sr.ht/~tsumanu/jirs

## Features

* Actor based asynchronous backend
* Ultra fast functional frontend build with WASM

## How to run it

### Config files

```toml
# web.toml
concurrency = 2
port = "5000"
bind = "0.0.0.0"
ssl = false
```

```toml
# db.toml
concurrency = 2
database_url = "postgres://postgres@localhost:5432/jirs"
```

```toml
# mail.toml
concurrency = 2
user = "apikey"
pass = "YOUR-TOKEN"
host = "smtp.sendgrid.net"
from = "contact@jirs.pl"
```

### Local variables

Within `jirs` directory place `.env` file with following content

```dotenv
DATABASE_URL=postgres://postgres@localhost:5432/jirs
RUST_LOG=actix_web=info,diesel=info
JIRS_CLIENT_PORT=7000
JIRS_CLIENT_BIND=0.0.0.0
JIRS_SERVER_PORT=5000
JIRS_SERVER_BIND=0.0.0.0
NODE_ENV=development
DEBUG=true
```

### Backend

Requirements:

* PostgreSQL

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration run

cargo run --bin jirs_server
```

### Frontend

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cd jirs_client
yarn
yarn webpack-dev-server
```

## Issue trackers

https://todo.sr.ht/~tsumanu/JIRS
