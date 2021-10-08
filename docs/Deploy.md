# Deploy with Nginx

You can deploy easily JIRS to any PC including Raspberry PI. To do this you will need compile it from source code.

We will use following setup, but you can modify it.

* `issues.example.com` - domain
* `postgres://postgres@192.168.1.144:5432/jirs` - database on other machine and within inner network

* `/var/jirs` - main directory
* `/var/jirs/clone` - cloned source code
* `/var/jirs/web` - All static assets including wasm library
* `/var/jirs/config` - config files
* `/var/jirs/uploads` - uploaded files

### JIRS Config files

* `config/db.toml` (REQUIRED)

```toml
concurrency = 2
database_url = "postgres://postgres@192.168.1.144:5432/jirs"
```

* `config/web.toml` (public_path is required)

```toml
concurrency = 2
port = "5000"
bind = "0.0.0.0"
ssl = false
tmp_dir = "/tmp"
public_path = "issues.example.com"
```

* `config/fs.toml` (must match nginx config)

```toml
store_path = "./uploads"
client_path = "/uploads"
tmp_path = "/tmp"
concurrency = 2
```

* `config/mail.toml` (REQUIRED)

```toml
concurrency = 2
user = "apikey"
pass = "SG.ARJL0wAxQk-LLJca9FJ5Lg.ahs7dyashd8a7shd7ahsd978h"
host = "smtp.sendgrid.net"
from = "admin@issues.example.com"
```

### NGINX config file

```nginx
server {
	listen 80;
	server_name issues.example.com;
	
	root /var/jirs/web;
	try_files $uri $uri/index.html index.html;

	location ~ /uploads/ {
		root /var/jirs;
	}

	location ~ .js {
		add_header 'Content-Type' 'application/javascript';
		add_header 'Access-Control-Allow-Origin' '*';
	}
	location ~ .css {
		add_header 'Content-Type' 'text/css';
	}
	location ~ .wasm {
		add_header 'Content-Type' 'application/wasm';
		add_header 'Access-Control-Allow-Origin' '*';
	}
	
	location /ws/ {
		proxy_pass http://localhost:5000;
		proxy_http_version 1.1;
		proxy_set_header Upgrade $http_upgrade;
		proxy_set_header Connection "Upgrade";
		proxy_set_header Host $host;
	}
	location /avatar {
		proxy_pass http://localhost:5000;
	}
	location ~ / {
		add_header 'Content-Type' 'text/html';
		add_header 'Access-Control-Allow-Origin' '*';
		root /var/jirs/web;
		try_files $uri $uri/index.html /index.html;
	}
}
```

### Compile application

```bash
cargo install --force wasm-pack
cargo install --force rsass
```

Ensure `$HOME/.cargo/bin` is in `$PATH`

* Compile server

```bash
cargo build --bin jirs_server --release --no-default-features --features local-storage
cp ./target/release/jirs_server /usr/bin/jirs_server
```

* Compile web client

```bash
./web/scripts/prod.sh

cp -r /tmp/wasm/* /var/jirs/web
```

If it fails (there is no wasm-opt for Raspberry PI) you must disable wasm-opt or compile it on any other PC and just
copy everything to `/var/jirs/web`


