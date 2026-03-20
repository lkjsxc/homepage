# Overview

The system is rebuilt as a Rust `actix-web` service with browser-side TypeScript assets.

## Goals
- Functional core for deterministic logic.
- Async-first request handling.
- Minimal runtime: Rust binary plus static files.

## Runtime Shape
- API endpoints under `/api/v1`.
- Static frontend served at `/` and `/assets/*`.
- In-memory concurrent state for background jobs.

## Non-goals
- No backward compatibility with the legacy C server.
