use crate::sampling::group::guard::SamplingGuard;
use crate::sampling::group::inner::Inner;
use crate::sampling::record::Record;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct FixedSamplingGroup {
    inner: Arc<RwLock<Inner>>,
}

impl FixedSamplingGroup {
    pub(crate) const fn new(inner: Arc<RwLock<Inner>>) -> Self {
        Self { inner }
    }

    fn inner(&self) -> RwLockReadGuard<'_, Inner> {
        self.inner.read().unwrap()
    }

    fn inner_mut(&self) -> RwLockWriteGuard<'_, Inner> {
        self.inner.write().unwrap()
    }

    pub fn enable(&self) -> io::Result<()> {
        self.inner().enable()
    }

    pub fn disable(&self) -> io::Result<()> {
        self.inner().disable()
    }

    pub fn next_record(&mut self, guard: &SamplingGuard) -> Option<Record> {
        self.inner_mut().next_record(guard.event_id())
    }
}
