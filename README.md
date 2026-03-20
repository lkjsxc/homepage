# Portfolio Service (Rust + Actix + TypeScript)

This repository is a docs-first rewrite of the previous C server.

## Stack
- Rust + `actix-web` backend
- Browser TypeScript frontend (transpiled at build time)
- Docker multi-stage build

## Features
- Async job creation and progress tracking
- Deterministic job result generation
- Health endpoint
- Static asset serving from Actix runtime

## API
- `GET /api/v1/health`
- `POST /api/v1/jobs`
- `GET /api/v1/jobs`
- `GET /api/v1/jobs/{id}`

## Local quick start
```sh
cargo run
```

Then open `http://localhost:8080`.

## Docker compose verification
```sh
docker compose build --no-cache checks portfolio
docker compose run --rm checks
docker compose up -d portfolio
curl -fsS http://localhost:8080/api/v1/health
```

## Documentation
- Root docs TOC: [docs/README.md](docs/README.md)
