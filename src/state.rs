use crate::model::{JobRecord, JobSnapshot};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Default)]
struct StateData {
    jobs: HashMap<String, JobRecord>,
    order: Vec<String>,
}

struct InnerState {
    next_sequence: AtomicU64,
    data: RwLock<StateData>,
}

#[derive(Clone)]
pub struct AppState {
    inner: Arc<InnerState>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(InnerState {
                next_sequence: AtomicU64::new(0),
                data: RwLock::new(StateData::default()),
            }),
        }
    }

    pub fn next_sequence(&self) -> u64 {
        self.inner.next_sequence.fetch_add(1, Ordering::Relaxed) + 1
    }

    pub fn insert_job(&self, record: JobRecord) {
        let mut guard = self.inner.data.write().unwrap_or_else(|error| error.into_inner());
        guard.order.push(record.snapshot.id.clone());
        guard.jobs.insert(record.snapshot.id.clone(), record);
    }

    pub fn with_job_mut<T>(&self, id: &str, action: impl FnOnce(&mut JobRecord) -> T) -> Option<T> {
        let mut guard = self.inner.data.write().unwrap_or_else(|error| error.into_inner());
        guard.jobs.get_mut(id).map(action)
    }

    pub fn job_snapshot(&self, id: &str) -> Option<JobSnapshot> {
        let guard = self.inner.data.read().unwrap_or_else(|error| error.into_inner());
        guard.jobs.get(id).map(|record| record.snapshot.clone())
    }

    pub fn list_snapshots(&self) -> Vec<JobSnapshot> {
        let guard = self.inner.data.read().unwrap_or_else(|error| error.into_inner());
        guard
            .order
            .iter()
            .rev()
            .filter_map(|id| guard.jobs.get(id).map(|record| record.snapshot.clone()))
            .collect()
    }
}
