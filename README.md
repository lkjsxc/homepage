# Portfolio Service (Rust + Actix + TypeScript)

This repository is a docs-first rewrite of the previous C server, now centered on a portfolio showcase homepage.

## Stack
- Rust + `actix-web` backend
- Browser TypeScript frontend (transpiled at build time)
- Docker multi-stage build

## Homepage Focus
- Profile section (name, headline, summary, location)
- Exactly 5 featured project cards
- Social links section
- Static asset serving from Actix runtime

## Supporting API
- `GET /api/v1/health`
- `POST /api/v1/jobs`
- `GET /api/v1/jobs`
- `GET /api/v1/jobs/{id}`

Async job endpoints remain available as supporting backend capabilities, but the primary UI at `/` is the portfolio homepage.

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
