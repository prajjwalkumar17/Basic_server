# Basic Server

[![Rust](https://img.shields.io/badge/rust-edition%202021-orange.svg)](https://www.rust-lang.org/)
[![Dependencies](https://img.shields.io/badge/dependencies-zero-blue.svg)](https://doc.rust-lang.org/std/)

A minimal HTTP/1.1 server built from scratch in Rust — no frameworks, no dependencies, just the standard library.

## Features

- **HTTP/1.1 request parsing** — method, path, query string, protocol validation
- **Static file serving** — serves files from a `public/` directory
- **Directory traversal protection** — blocks path traversal attempts
- **Custom routing** — handles `/`, `/hello`, and falls back to static files
- **404 handling** — returns proper status codes for missing routes
- **Query string support** — parses `?key=value&key2=value2` parameters
- **Zero dependencies** — uses only `std` library

## Project Structure

```
├── Cargo.toml              # Workspace definition
├── crates/
│   ├── basic_server/       # Binary entry point
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs     # Server startup & WebsiteHandler
│   └── basic_server_lib/   # Library
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── server.rs   # TCP listener & connection handling
│           ├── handler.rs  # Handler trait
│           └── http/
│               ├── mod.rs          # Public re-exports
│               ├── method.rs       # HTTP method enum (GET, etc.)
│               ├── request.rs      # Request parsing from raw bytes
│               ├── response.rs     # Response construction & sending
│               ├── query_string.rs # Query string parsing
│               └── status_code.rs  # Status codes (200, 400, 404)
├── public/
│   ├── index.html          # Served at /
│   ├── hello.html
│   └── style.css           # Served at /style.css
└── scripts/
    └── test-curls.sh       # Automated endpoint tests
```

## Getting Started

### Prerequisites

- [Rust](https://rust-lang.org/tools/install) (edition 2021)

### Run the Server

```bash
cargo run
```

The server starts on `127.0.0.1:8080` and serves static files from the `public/` directory.

### Configure Public Path

Override the static file directory with the `PUBLIC_PATH` environment variable:

```bash
PUBLIC_PATH=/path/to/static/files cargo run
```

## API Endpoints

| Method | Path     | Description                          | Status     |
|--------|----------|--------------------------------------|------------|
| GET    | `/`      | Serves `public/index.html`           | 200        |
| GET    | `/hello` | Returns `<h1>Hello there!</h1>`      | 200        |
| GET    | `/*`     | Serves matching file from `public/`  | 200 or 404 |
| *      | `/*`     | All other methods                    | 404        |

### Example Requests

```bash
# Homepage - serves index.html
curl http://127.0.0.1:8080/

# Hello endpoint - hardcoded greeting
curl http://127.0.0.1:8080/hello

# Static file
curl http://127.0.0.1:8080/style.css

# 404 for missing routes
curl -i http://127.0.0.1:8080/does-not-exist
```

## Testing

Run the automated curl smoke tests:

```bash
bash ./scripts/test-curls.sh
```

## License

This project is open source. Check the repository for license details.
