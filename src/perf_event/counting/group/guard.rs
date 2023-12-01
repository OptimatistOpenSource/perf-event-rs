use crate::counting::{Counting, CountingResult};
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct CountingGuard {
    pub counting: Arc<RwLock<Counting>>,
}

impl CountingGuard {
    pub(crate) fn new(counting: Arc<RwLock<Counting>>) -> Self {
        Self { counting }
    }

    pub(crate) fn as_counting(&self) -> RwLockReadGuard<'_, Counting> {
        self.counting.read().unwrap()
    }

    pub(crate) fn as_counting_mut(&self) -> RwLockWriteGuard<'_, Counting> {
        self.counting.write().unwrap()
    }

    pub fn result(&mut self) -> io::Result<CountingResult> {
        self.as_counting_mut().result()
    }
}
