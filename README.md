# Basic Server

A lightweight HTTP server written from scratch in Rust — no frameworks, no dependencies. Serves static files and handles basic routes.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- `cargo` (comes with Rust)

## Quick Start

```bash
git clone https://github.com/prajjwalkumar17/Basic_server.git
cd Basic_server
cargo run
```

The server starts on `http://127.0.0.1:8080`. You'll see `Starting server` in the terminal.

## Configuration

| Env Variable   | Default                      | Description                  |
|----------------|------------------------------|------------------------------|
| `PUBLIC_PATH`  | `<crate-root>/public`        | Path to static file directory |

```bash
# Serve files from a custom directory
PUBLIC_PATH=/var/www cargo run
```

## Endpoints

| Method | Path           | Description                        |
|--------|----------------|------------------------------------|
| GET    | `/`            | Serves `public/index.html`         |
| GET    | `/hello`       | Returns a greeting message         |
| GET    | `/style.css`   | Serves static CSS file             |
| GET    | `/<file>`      | Serves any file from `PUBLIC_PATH` |
| ANY    | (anything else)| Returns **404 Not Found**          |

## Testing the Endpoints

### Manual (curl)

```bash
# Server must be running in another terminal

curl -i http://127.0.0.1:8080/            # 200 — index page
curl -i http://127.0.0.1:8080/hello       # 200 — greeting
curl -i http://127.0.0.1:8080/style.css   # 200 — static CSS
curl -i http://127.0.0.1:8080/no-route    # 404
```

### Automated (script)

Runs all checks in one shot — starts the server, tests each endpoint, then shuts down:

```bash
./scripts/test-curls.sh
```

Expected output:

```
OK: / -> HTTP 200
OK: /hello -> HTTP 200
OK: /style.css -> HTTP 200
OK: /does-not-exist -> HTTP 404
All curl checks passed
```

## Project Structure

```
.
├── public/              # Static files served by the server
│   ├── index.html
│   ├── hello.html
│   └── style.css
├── scripts/
│   └── test-curls.sh    # Automated endpoint tests
├── src/
│   ├── main.rs          # Entry point — binds to 127.0.0.1:8080
│   ├── server.rs        # TCP listener & connection handling
│   ├── website_handler.rs  # Route matching & file serving
│   └── http/            # Minimal HTTP parser
│       ├── method.rs
│       ├── request.rs
│       ├── response.rs
│       ├── status_code.rs
│       └── query_string.rs
└── Cargo.toml
```

## License

MIT
