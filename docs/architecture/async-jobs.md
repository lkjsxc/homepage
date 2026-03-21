# Async Job Pipeline

This pipeline documents supporting API behavior and is not the primary homepage UX.

## Flow
1. Client creates a job with `POST /api/v1/jobs`.
2. Server persists an initial snapshot in concurrent state.
3. Server spawns an async worker task.
4. Worker updates progress and final result.
5. Client polls `GET /api/v1/jobs/{id}`.

## Properties
- Non-blocking request path.
- Concurrent-safe updates with lock-minimized data structures.
- Bounded validation for user-supplied workload.

## Result Model
- `queued` -> `running` -> `completed` or `failed`.
- Progress is integer percent from `0` to `100`.
