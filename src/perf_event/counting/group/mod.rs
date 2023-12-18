mod fixed;
mod guard;
mod inner;
mod result;

use crate::counting::group::guard::CountingGuard;
use crate::counting::Config;
use crate::infra::WrapResult;
use libc::pid_t;
pub use result::*;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::counting::group::inner::Inner;
pub use fixed::*;
pub use guard::*;
pub use result::*;

pub struct CountingGroup {
    pid: pid_t,
    cpu: i32,
    inner: Arc<RwLock<Inner>>,
}

impl CountingGroup {
    pub(crate) unsafe fn new(pid: pid_t, cpu: i32) -> Self {
        Self {
            pid,
            cpu,
            inner: Arc::new(RwLock::new(Inner::new())),
        }
    }

    #[allow(dead_code)]
    fn inner(&self) -> RwLockReadGuard<'_, Inner> {
        self.inner.read().unwrap()
    }

    fn inner_mut(&self) -> RwLockWriteGuard<'_, Inner> {
        self.inner.write().unwrap()
    }

    pub fn add_member(&mut self, cfg: &Config) -> io::Result<CountingGuard> {
        let event_id = self.inner_mut().add_member(self.cpu, self.pid, cfg)?;
        CountingGuard::new(event_id, self.inner.clone()).wrap_ok()
    }

    pub fn enable(self) -> io::Result<FixedCountingGroup> {
        self.inner_mut().enable()?;
        FixedCountingGroup::new(self.inner).wrap_ok()
    }

    pub fn result(&mut self) -> io::Result<CountingGroupResult> {
        self.inner_mut().result()
    }
}
