# Overview

The system is rebuilt as a Rust `actix-web` service with browser-side TypeScript assets and a portfolio showcase homepage.

## Goals
- Functional core for deterministic logic.
- Portfolio-first homepage delivery.
- Minimal runtime: Rust binary plus static files.

## Runtime Shape
- API endpoints under `/api/v1`.
- Static frontend served at `/` and `/assets/*`, with `/` focused on profile + five featured projects + social links.
- In-memory concurrent state for supporting async job APIs.

## Non-goals
- No backward compatibility with the legacy C server.
