# Contracts

## Create Job Request
```json
{ "label": "string", "steps": 32, "seed": "optional" }
```

## Create Job Response
```json
{ "jobId": "job-42" }
```

## Job Snapshot
```json
{
  "id": "job-42",
  "label": "demo",
  "status": "running",
  "progress": 56,
  "steps": 32,
  "message": "step 18/32",
  "result": null
}
```
