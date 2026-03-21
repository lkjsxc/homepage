# Module Map

## Rust
- `src/main.rs`: bootstrapping and route wiring.
- `src/config.rs`: environment parsing.
- `src/model.rs`: domain and API DTOs.
- `src/state.rs`: shared concurrent app state.
- `src/job_engine.rs`: async job execution.
- `src/api.rs`: HTTP handlers.

## Frontend
- `frontend/src/types.ts`: portfolio content domain types.
- `frontend/src/content.ts`: typed portfolio showcase content source.
- `frontend/src/job-types.ts`: async-job transport types for supporting API integrations.
- `frontend/src/api.ts`: async-job fetch wrappers used by non-homepage workflows.
- `frontend/src/app.ts`: homepage renderer for profile, 5 featured projects, and social links.
