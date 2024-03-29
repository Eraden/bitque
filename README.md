# A simplified Jira clone built with seed.rs and actix

![JIRS](https://raw.githubusercontent.com/Eraden/jirs/master/web/static/project-icon.svg)

Server: [![builds.sr.ht status](https://builds.sr.ht/~tsumanu/jirs/server.yml.svg)](https://builds.sr.ht/~tsumanu/jirs/server.yml?)
Client: [![builds.sr.ht status](https://builds.sr.ht/~tsumanu/jirs/client.yml.svg)](https://builds.sr.ht/~tsumanu/jirs/client.yml?)

Main repo: https://git.sr.ht/~tsumanu/jirs

Demo: https://jirs.ita-prog.pl

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

## Known bugs

* Bad sorting when dragging up and down

## Roadmap

##### Version 1.0

* Basic issue management
* Basic columns management
* Basic user management

##### Version 1.1

* Add Epic
* Add grouping by Epic
* Add backend maximal per seconds request or die
* Add fibonacci tracked issue reports
* Add hourly tracked issue reports
* Add Rich Text Editor
* Add personal settings to choose MDE (Markdown Editor) or RTE
* Add issues and filters

##### Version 1.1.1

* Refactor actors
* Extract code highlight to server actor
* Handle upload avatar with stream
* Move config to `./config` directory
* Fix S3 upload with upgraded version of `rusoto`
* Remove Custom Elements
* Replace CSS with SCSS
* Disable RTE until properly optimized 

##### Work Progress

* [X] Add Epic
* [X] Edit Epic
* [X] Delete Epic
* [X] Epic `starts` and `ends` date
* [X] Grouping by Epic
* [X] Basic Rich Text Editor
* [ ] Insert Code in Rich Text Editor
* [X] Code syntax
* [X] Personal settings to choose MDE (Markdown Editor) or RTE
* [X] Issues and filters view
* [X] Issues and filters working filters

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
./web/scripts/prod.sh
```

```bash
sudo ln -s ./jirs.nginx /etc/nginx/sites-enabled/
sudo nginx -s reload
```

## Issue trackers

https://todo.sr.ht/~tsumanu/JIRS

## Details

### Display code syntax

Custom element glued with WASM

* `file-path` have connected on attr changed callback and will change displayed path
* `lang` does not have callback and it's used only on `connectedCallback`

```html
<jirs-code-view lang="Rust" file-path="/some/path.rs">
struct Foo {
}
</jirs-code-view>
```

### Supported languages

* ASP
* AWK
* ActionScript
* Advanced CSV
* AppleScript
* Assembly x86 (NASM)
* Batch File
* BibTeX
* Bourne Again Shell (bash)
* C
* C#
* C++
* CMake
* CMake C Header
* CMake C++ Header
* CMakeCache
* CMakeCommands
* CSS
* Cargo Build Results
* Clojure
* Crystal
* D
* DMD Output
* Dart
* Diff
* Dockerfile
* Elixir
* Elm
* Elm Compile Messages
* Elm Documentation
* Erlang
* F#
* Fortran (Fixed Form)
* Fortran (Modern)
* Fortran Namelist
* Friendly Interactive Shell (fish)
* GFortran Build Results
* Generic Config
* Git Attributes
* Git Commit
* Git Common
* Git Config
* Git Ignore
* Git Link
* Git Log
* Git Mailmap
* Git Rebase Todo
* Go
* GraphQL
* Graphviz (DOT)
* Groovy
* HTML
* HTML (ASP)
* HTML (EEx)
* HTML (Erlang)
* HTML (Jinja2)
* HTML (Rails)
* HTML (Tcl)
* Handlebars
* Haskell
* JSON
* Java
* Java Properties
* Java Server Page (JSP)
* JavaScript
* JavaScript (Rails)
* Javadoc
* Jinja2
* Julia
* Kotlin
* LaTeX
* LaTeX Log
* Less
* Linker Script
* Lisp
* Literate Haskell
* Lua
* MATLAB
* Make Output
* Makefile
* Markdown
* MiniZinc (MZN)
* MultiMarkdown
* NAnt Build File
* Nim
* Nix
* OCaml
* OCamllex
* OCamlyacc
* Objective-C
* Objective-C++
* OpenMP (Fortran)
* PHP
* PHP Source
* Pascal
* Perl
* Plain Text
* PowerShell
* PureScript
* Python
* R
* R Console
* Racket
* Rd (R Documentation)
* Reason
* Regular Expression
* Regular Expressions (Elixir)
* Regular Expressions (Javascript)
* Regular Expressions (PHP)
* Regular Expressions (Python)
* Ruby
* Ruby Haml
* Ruby on Rails
* Rust
* SCSS
* SQL
* SQL (Rails)
* SWI-Prolog
* Sass
* Scala
* Shell-Unix-Generic
* Stylus
* Swift
* TOML
* Tcl
* TeX
* Textile
* TypeScript
* TypeScriptReact
* VimL
* XML
* YAML
* camlp4
* commands-builtin-shell-bash
* lrc
* reStructuredText
* srt
