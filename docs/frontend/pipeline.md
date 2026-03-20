# Build Pipeline

TypeScript is transpiled during Docker build.

## Inputs
- `frontend/src/*.ts`
- `frontend/static/index.html`
- `frontend/static/styles.css`

## Outputs
- `frontend/dist/app.js`
- `frontend/dist/index.html`
- `frontend/dist/styles.css`

Only generated assets are copied into the runtime image.
