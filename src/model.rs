use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize)]
pub struct ErrorEnvelope {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobSnapshot {
    pub id: String,
    pub label: String,
    pub status: JobStatus,
    pub progress: u8,
    pub steps: u32,
    pub message: String,
    pub result: Option<String>,
}

#[derive(Debug, Clone)]
pub struct JobRecord {
    pub snapshot: JobSnapshot,
    pub seed: Option<String>,
}

impl JobRecord {
    pub fn queued(id: String, label: String, steps: u32, seed: Option<String>) -> Self {
        Self {
            snapshot: JobSnapshot {
                id,
                label,
                status: JobStatus::Queued,
                progress: 0,
                steps,
                message: "queued".to_owned(),
                result: None,
            },
            seed,
        }
    }

    pub fn mark_running(&mut self) {
        self.snapshot.status = JobStatus::Running;
        self.snapshot.message = "running".to_owned();
    }

    pub fn mark_progress(&mut self, step: u32, total_steps: u32) {
        self.snapshot.progress = progress_for(step, total_steps);
        self.snapshot.message = step_message(step, total_steps);
    }

    pub fn mark_completed(&mut self, result: String) {
        self.snapshot.status = JobStatus::Completed;
        self.snapshot.progress = 100;
        self.snapshot.message = "completed".to_owned();
        self.snapshot.result = Some(result);
    }

    pub fn mark_failed(&mut self, message: String) {
        self.snapshot.status = JobStatus::Failed;
        self.snapshot.message = message;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobCreateRequest {
    pub label: String,
    pub steps: u32,
    #[serde(default)]
    pub seed: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobCreateResponse {
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

impl HealthResponse {
    pub fn ok() -> Self {
        Self {
            status: "ok",
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}

pub fn clean_label(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        "untitled-job".to_owned()
    } else {
        trimmed.chars().take(64).collect()
    }
}

pub fn normalize_seed(raw: Option<&str>) -> Option<String> {
    raw.map(str::trim)
        .filter(|seed| !seed.is_empty())
        .map(|seed| seed.chars().take(128).collect())
}

pub fn validate_steps(steps: u32, max_steps: u32) -> Result<u32, ApiError> {
    if steps == 0 || steps > max_steps {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "invalid_steps",
            format!("steps must be 1..={max_steps}"),
        ));
    }
    Ok(steps)
}

pub fn make_job_id(sequence: u64) -> String {
    format!("job-{sequence}")
}

pub fn progress_for(step: u32, total_steps: u32) -> u8 {
    ((step.saturating_mul(100)) / total_steps.max(1)).min(100) as u8
}

pub fn step_message(step: u32, total_steps: u32) -> String {
    format!("step {step}/{total_steps}")
}

pub fn finalize_result(label: &str, seed: Option<&str>, steps: u32) -> String {
    let mut hasher = DefaultHasher::new();
    label.hash(&mut hasher);
    seed.unwrap_or_default().hash(&mut hasher);
    steps.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
