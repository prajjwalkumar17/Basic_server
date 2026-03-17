# Basic Server

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
├── Cargo.toml
├── public/
│   ├── index.html       # Served at /
│   ├── hello.html
│   └── style.css        # Served at /style.css
├── scripts/
│   └── test-curls.sh    # Automated endpoint tests
└── src/
    ├── main.rs              # Entry point, configures server
    ├── server.rs            # TCP listener & connection handling
    ├── website_handler.rs   # Route logic & static file serving
    └── http/
        ├── mod.rs           # Public re-exports
        ├── method.rs        # HTTP method enum (GET, etc.)
        ├── request.rs       # Request parsing from raw bytes
        ├── response.rs      # Response construction & sending
        ├── query_string.rs  # Query string parsing
        └── status_code.rs   # Status codes (200, 400, 404)
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

| Method | Path      | Description                        | Status |
|--------|-----------|------------------------------------|--------|
| GET    | `/`       | Serves `public/index.html`         | 200    |
| GET    | `/hello`  | Returns a greeting message         | 200    |
| GET    | `/*`      | Serves matching file from `public/` | 200/404 |
| *      | `/*`      | All other methods                  | 404    |

### Example Requests

```bash
# Homepage
curl http://127.0.0.1:8080/

# Hello endpoint
curl http://127.0.0.1:8080/hello

# Static file
curl http://127.0.0.1:8080/style.css

# Query string support
curl "http://127.0.0.1:8080/hello?name=world"

# 404 for missing routes
curl http://127.0.0.1:8080/does-not-exist
```

## Testing

Run the automated curl smoke tests (starts the server, tests all endpoints, cleans up):

```bash
bash ./scripts/test-curls.sh
```

The script validates:
- `/` returns 200 with index page content
- `/hello` returns 200 with greeting
- `/style.css` returns 200 with CSS content
- Missing routes return 404

## License

This project is open source. Check the repository for license details.
