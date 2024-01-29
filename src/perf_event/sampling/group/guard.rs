use crate::infra::WrapResult;
use crate::sampling::group::inner::Inner;
use crate::sampling::record::Record;
use crate::sampling::SamplerStat;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct SamplerGuard {
    event_id: u64,
    inner: Arc<RwLock<Inner>>,
}

impl SamplerGuard {
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

    pub fn next_record(&mut self) -> Option<Record> {
        self.as_inner_mut().next_record(self.event_id)
    }

    pub fn stat(&mut self) -> io::Result<SamplerStat> {
        let result = self.as_inner_mut().stat()?;
        let member_count = result.member_count(self)?;
        SamplerStat {
            event_id: self.event_id,
            event_count: member_count.event_count,
            #[cfg(feature = "linux-6.0")]
            event_lost: member_count.event_lost,
            time_enabled: result.time_enabled,
            time_running: result.time_running,
        }
        .wrap_ok()
    }
}

impl Iterator for SamplerGuard {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_record()
    }
}
