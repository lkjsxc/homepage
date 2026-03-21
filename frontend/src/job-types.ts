export type JobStatus = "queued" | "running" | "completed" | "failed";

export interface JobSnapshot {
  id: string;
  label: string;
  status: JobStatus;
  progress: number;
  steps: number;
  message: string;
  result: string | null;
}

export interface JobCreateRequest {
  label: string;
  steps: number;
  seed?: string;
}

export interface JobCreateResponse {
  jobId: string;
}

export interface ErrorEnvelope {
  code: string;
  message: string;
}
