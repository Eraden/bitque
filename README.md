# A simplified Jira clone built with seed.rs and actix

Server: [![builds.sr.ht status](https://builds.sr.ht/~tsumanu/jirs/server.yml.svg)](https://builds.sr.ht/~tsumanu/jirs/server.yml?)
Client: [![builds.sr.ht status](https://builds.sr.ht/~tsumanu/jirs/client.yml.svg)](https://builds.sr.ht/~tsumanu/jirs/client.yml?)

https://git.sr.ht/~tsumanu/jirs

## Features

* Actor based asynchronous backend
* Ultra fast functional frontend build with WASM
* Showing message when person is invited to project
* Send e-mail with invitation to project
* Switch project in profile page
* Choose time tracking in project
* Edit, delete and move issues
* Comment issue
* Add people to project

## How to run it

### Config files

#### WEB

```toml
# web.toml
concurrency = 2
port = "5000"
bind = "0.0.0.0"
ssl = false
tmp_dir = "./tmp"

[s3]
access_key_id = ""
secret_access_key = ""
bucket = ""
region_name = "eu-central-1"

[filesystem]
store_path = ""
client_path = "/img"
```

##### Upload local storage

If default feature `"local-storage"` is on your uploaded files will be stored on your machine.
This requires additional configuration.

```toml
[filesystem]
store_path = "/var/jirs/uploads"
client_path = "/img"
```

* `store_path` is your local machine path. Files will be saved there. This can be relative to `CWD` path or absolute path.
* `client_path` is web path

Both must be set and non-empty

##### Upload to AWS S3

If default feature `"aws-s3"` is on your uploaded files will be send to AWS S3 service.
This requires additional configuration.

```toml
[s3]
access_key_id = ""
secret_access_key = ""
bucket = ""
region_name = "eu-central-1"
```

#### Database

```toml
# db.toml
concurrency = 2
database_url = "postgres://postgres@localhost:5432/jirs"
```

#### Mail Service

You can send e-mail only via service which will handle this. This application was build using sendgrid.

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

Client and Server bind/port must be provided. Client will be build using those variables and will send requests only using this address.
`DATABASE_URL` is required only to setup database. Runtime will use `db.toml`.

### Backend

Requirements:

* PostgreSQL

```bash
cargo install diesel_cli --no-default-features --features postgres
export DATABASE_URL=postgres://postgres@localhost/jirs
diesel setup
diesel migration run

cargo run --bin jirs_server
```

### Frontend

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cd jirs_client
yarn
./scripts/prod.sh
```

```bash
sudo ln -s ./jirs.nginx /etc/nginx/sites-enabled/
sudo nginx -s reload
```

## Issue trackers

https://todo.sr.ht/~tsumanu/JIRS
