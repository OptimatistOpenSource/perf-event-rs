use crate::counting::group::inner::Inner;
use crate::counting::CountingGroupResult;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct FixedCountingGroup {
    inner: Arc<RwLock<Inner>>,
}

impl FixedCountingGroup {
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

    pub fn result(&mut self) -> io::Result<CountingGroupResult> {
        self.inner_mut().result()
    }
}
