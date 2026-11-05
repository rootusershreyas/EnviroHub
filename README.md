# EnviroHub

A shared environment booking API built as one Rust application. This first
milestone provides Member A's application foundation.

## Run

Install stable Rust, then run:

```sh
cargo run
```

The server defaults to `127.0.0.1:3000`. Configure it with environment variables:

```sh
BIND_ADDR=0.0.0.0:3000 RUST_LOG=envirohub=info,tower_http=debug cargo run
```

`.env.example` documents the settings; the application reads process environment
variables and does not automatically load a `.env` file.

In another terminal:

```sh
curl http://127.0.0.1:3000/health
# {"status":"ok"}
```

`GET /health` is a process health check and requires no database. Logs are JSON.
Ctrl+C or SIGTERM initiates graceful shutdown.

## Verify

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Integration boundaries

- `src/app.rs` exposes `router()` for application startup and HTTP tests. Add
  feature routers under `/api/v1` here.
- `src/error.rs` maps shared application errors to JSON containing `code` and
  `message`, with HTTP 400, 404, 409, and 500 responses.
- Member B adds SQLx, PostgreSQL configuration, migrations, and database readiness.
  Coordinate adding a shared `AppState` containing the pool at that point.
- Member A adds programmers, teams, memberships, and lead rules next.
- Member C adds environments and the booking lifecycle.

Authentication, database access, and feature endpoints are not part of this
foundation milestone.
