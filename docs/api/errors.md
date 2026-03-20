# Errors

All error responses use a JSON envelope.

```json
{ "code": "invalid_steps", "message": "steps must be 1..=4096" }
```

## Typical Codes
- `invalid_payload`
- `invalid_steps`
- `job_not_found`
- `internal_error`
