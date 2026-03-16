# Basic_server

OSS equivalent

## Curl smoke tests

Run from repo root:

```bash
cargo run
```

In another terminal run:

```bash
./scripts/test-curls.sh
```

Or run everything in one shot:

```bash
bash ./scripts/test-curls.sh
```

The script starts the server, checks:
- `/` for index page
- `/hello` for greeting response
- `/style.css` for static file serving
- missing route returns 404
