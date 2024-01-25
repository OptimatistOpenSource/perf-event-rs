use crate::counting::group::inner::Inner;
use crate::counting::CounterGroupStat;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct FixedCounterGroup {
    inner: Arc<RwLock<Inner>>,
}

impl FixedCounterGroup {
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

    pub fn reset_count(&self) -> io::Result<()> {
        self.inner().reset_count()
    }

    pub fn stat(&mut self) -> io::Result<CounterGroupStat> {
        self.inner_mut().stat()
    }
}
