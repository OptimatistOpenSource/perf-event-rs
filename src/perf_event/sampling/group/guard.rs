use crate::sampling::group::inner::Inner;
use crate::sampling::record::Record;
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
}

impl Iterator for SamplerGuard {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_record()
    }
}
