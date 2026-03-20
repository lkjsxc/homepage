# Verification Flow

Use these commands:

```sh
docker compose build --no-cache checks portfolio
docker compose run --rm checks
docker compose up -d portfolio
curl -fsS http://localhost:8080/api/v1/health
```

Stop services after checks:

```sh
docker compose down
```
