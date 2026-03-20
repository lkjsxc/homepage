"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.listJobs = exports.createJob = void 0;
const JSON_HEADERS = { "Content-Type": "application/json" };
const parseOrThrow = async (response) => {
    if (response.ok) {
        return (await response.json());
    }
    let payload = null;
    try {
        payload = (await response.json());
    }
    catch {
        payload = null;
    }
    const reason = payload?.message ?? `request failed with ${response.status}`;
    throw new Error(reason);
};
const createJob = async (input) => {
    const response = await fetch("/api/v1/jobs", {
        method: "POST",
        headers: JSON_HEADERS,
        body: JSON.stringify(input),
    });
    return parseOrThrow(response);
};
exports.createJob = createJob;
const listJobs = async () => {
    const response = await fetch("/api/v1/jobs");
    return parseOrThrow(response);
};
exports.listJobs = listJobs;
