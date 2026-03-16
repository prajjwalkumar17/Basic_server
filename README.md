# Basic_server

OSS equivalent

## Curl smoke tests

Run the script below to quickly verify server responses end-to-end:

```bash
./scripts/test-curls.sh
```

The script starts the server, checks:
- `/` for index page
- `/hello` for greeting response
- `/style.css` for static file serving
- missing route returns 404
