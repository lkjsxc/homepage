"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const api_js_1 = require("./api.js");
const POLL_MS = 500;
const byId = (id) => {
    const node = document.getElementById(id);
    if (!node) {
        throw new Error(`missing element #${id}`);
    }
    return node;
};
const form = byId("job-form");
const labelInput = byId("label");
const stepsInput = byId("steps");
const seedInput = byId("seed");
const errorBox = byId("error");
const jobsList = byId("jobs");
let timer = null;
const renderJob = (job) => {
    const result = job.result ? `<code>${job.result}</code>` : "<em>pending</em>";
    return [
        `<strong>${job.label}</strong> <span class="status">${job.status}</span>`,
        `<span>${job.message}</span>`,
        `<span>${job.progress}% (${job.steps} steps)</span>`,
        `<span>result: ${result}</span>`,
    ].join("<br>");
};
const renderJobs = (jobs) => {
    jobsList.innerHTML = jobs
        .map((job) => `<li class="job-card">${renderJob(job)}</li>`)
        .join("");
};
const schedulePoll = () => {
    if (timer !== null) {
        return;
    }
    timer = window.setTimeout(async () => {
        timer = null;
        await refreshJobs();
    }, POLL_MS);
};
const refreshJobs = async () => {
    const jobs = await (0, api_js_1.listJobs)();
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
        await (0, api_js_1.createJob)({
            label,
            steps,
            seed: seed.length > 0 ? seed : undefined,
        });
        form.reset();
        await refreshJobs();
    }
    catch (error) {
        const message = error instanceof Error ? error.message : "unknown client error";
        errorBox.textContent = message;
    }
});
void refreshJobs();
