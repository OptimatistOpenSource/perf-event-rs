use crate::counting::group::inner::Inner;
use crate::counting::CountingResult;
use crate::infra::WrapResult;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct CountingGuard {
    event_id: u64,
    inner: Arc<RwLock<Inner>>,
}

impl CountingGuard {
    pub(crate) fn new(event_id: u64, inner: Arc<RwLock<Inner>>) -> Self {
        Self { event_id, inner }
    }

    pub fn as_inner(&self) -> RwLockReadGuard<'_, Inner> {
        self.inner.read().unwrap()
    }

    pub fn as_inner_mut(&self) -> RwLockWriteGuard<'_, Inner> {
        self.inner.write().unwrap()
    }

    pub const fn event_id(&self) -> u64 {
        self.event_id
    }

    pub fn result(&mut self) -> io::Result<CountingResult> {
        let result = self.as_inner_mut().result()?;
        CountingResult {
            event_count: result.member_count(self)?,
            time_enabled: result.time_enabled,
            time_running: result.time_running,
        }
        .wrap_ok()
    }
}
