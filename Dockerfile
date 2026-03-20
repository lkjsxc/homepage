FROM node:20-bookworm-slim AS frontend-build
WORKDIR /workspace/frontend
COPY frontend/package.json ./
RUN npm install --no-fund --no-audit --silent
COPY frontend/ ./
RUN npm run build

FROM rust:1.88-bookworm AS backend-build
WORKDIR /workspace
COPY Cargo.toml ./
COPY src/ ./src/
RUN cargo test --release
RUN cargo build --release

FROM backend-build AS checks
COPY --from=frontend-build /workspace/frontend/dist /workspace/frontend/dist
CMD ["/bin/sh", "-c", "cargo test --release && test -f /workspace/frontend/dist/assets/app.js && test -f /workspace/frontend/dist/index.html && echo checks-passed"]

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=backend-build /workspace/target/release/portfolio /app/portfolio
COPY --from=frontend-build /workspace/frontend/dist /app/frontend
ENV HOST=0.0.0.0
ENV PORT=8080
ENV JOB_MAX_STEPS=4096
ENV JOB_TICK_MS=20
ENV FRONTEND_DIR=/app/frontend
EXPOSE 8080
CMD ["/app/portfolio"]
