import type {
  ErrorEnvelope,
  JobCreateRequest,
  JobCreateResponse,
  JobSnapshot,
} from "./job-types.js";

const JSON_HEADERS = { "Content-Type": "application/json" } as const;

const parseOrThrow = async <T>(response: Response): Promise<T> => {
  if (response.ok) {
    return (await response.json()) as T;
  }

  let payload: ErrorEnvelope | null = null;
  try {
    payload = (await response.json()) as ErrorEnvelope;
  } catch {
    payload = null;
  }

  const reason = payload?.message ?? `request failed with ${response.status}`;
  throw new Error(reason);
};

export const createJob = async (
  input: JobCreateRequest,
): Promise<JobCreateResponse> => {
  const response = await fetch("/api/v1/jobs", {
    method: "POST",
    headers: JSON_HEADERS,
    body: JSON.stringify(input),
  });
  return parseOrThrow<JobCreateResponse>(response);
};

export const listJobs = async (): Promise<JobSnapshot[]> => {
  const response = await fetch("/api/v1/jobs");
  return parseOrThrow<JobSnapshot[]>(response);
};
