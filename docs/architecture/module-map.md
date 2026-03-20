# Module Map

## Rust
- `src/main.rs`: bootstrapping and route wiring.
- `src/config.rs`: environment parsing.
- `src/model.rs`: domain and API DTOs.
- `src/state.rs`: shared concurrent app state.
- `src/job_engine.rs`: async job execution.
- `src/api.rs`: HTTP handlers.

## Frontend
- `frontend/src/types.ts`: shared client types.
- `frontend/src/api.ts`: fetch wrappers.
- `frontend/src/app.ts`: UI interactions and polling.
