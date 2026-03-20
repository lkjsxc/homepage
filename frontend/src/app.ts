import { createJob, listJobs } from "./api.js";
import type { JobSnapshot } from "./types.js";

const POLL_MS = 500;

const byId = <T extends HTMLElement>(id: string): T => {
  const node = document.getElementById(id);
  if (!node) {
    throw new Error(`missing element #${id}`);
  }
  return node as T;
};

const form = byId<HTMLFormElement>("job-form");
const labelInput = byId<HTMLInputElement>("label");
const stepsInput = byId<HTMLInputElement>("steps");
const seedInput = byId<HTMLInputElement>("seed");
const errorBox = byId<HTMLParagraphElement>("error");
const jobsList = byId<HTMLUListElement>("jobs");

let timer: number | null = null;

const renderJob = (job: JobSnapshot): string => {
  const result = job.result ? `<code>${job.result}</code>` : "<em>pending</em>";
  return [
    `<strong>${job.label}</strong> <span class="status">${job.status}</span>`,
    `<span>${job.message}</span>`,
    `<span>${job.progress}% (${job.steps} steps)</span>`,
    `<span>result: ${result}</span>`,
  ].join("<br>");
};

const renderJobs = (jobs: JobSnapshot[]): void => {
  jobsList.innerHTML = jobs
    .map((job) => `<li class="job-card">${renderJob(job)}</li>`)
    .join("");
};

const schedulePoll = (): void => {
  if (timer !== null) {
    return;
  }
  timer = window.setTimeout(async () => {
    timer = null;
    await refreshJobs();
  }, POLL_MS);
};

const refreshJobs = async (): Promise<void> => {
  const jobs = await listJobs();
  renderJobs(jobs);
  if (jobs.some((job) => job.status === "queued" || job.status === "running")) {
    schedulePoll();
  }
};

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  errorBox.textContent = "";

  const label = labelInput.value;
  const steps = Number.parseInt(stepsInput.value, 10);
  const seed = seedInput.value.trim();

  try {
    await createJob({
      label,
      steps,
      seed: seed.length > 0 ? seed : undefined,
    });
    form.reset();
    await refreshJobs();
  } catch (error) {
    const message = error instanceof Error ? error.message : "unknown client error";
    errorBox.textContent = message;
  }
});

void refreshJobs();
