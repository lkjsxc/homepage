use crate::model::finalize_result;
use crate::state::AppState;
use tokio::time::{sleep, Duration};

fn should_force_failure(seed: Option<&str>) -> bool {
    matches!(seed, Some(value) if value.eq_ignore_ascii_case("fail"))
}

pub fn spawn_job(state: AppState, job_id: String, tick_ms: u64) {
    tokio::spawn(async move {
        let (steps, label, seed) = match state.with_job_mut(&job_id, |job| {
            job.mark_running();
            (job.snapshot.steps, job.snapshot.label.clone(), job.seed.clone())
        }) {
            Some(data) => data,
            None => return,
        };

        if should_force_failure(seed.as_deref()) {
            let _ = state.with_job_mut(&job_id, |job| {
                job.mark_failed("forced failure requested by seed".to_owned())
            });
            return;
        }

        for step in 1..=steps {
            sleep(Duration::from_millis(tick_ms)).await;
            if state
                .with_job_mut(&job_id, |job| job.mark_progress(step, steps))
                .is_none()
            {
                return;
            }
        }

        let result = finalize_result(&label, seed.as_deref(), steps);
        if state
            .with_job_mut(&job_id, |job| job.mark_completed(result))
            .is_none()
        {
            return;
        }
    });
}
