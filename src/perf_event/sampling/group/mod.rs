mod fixed;
mod guard;
mod inner;
mod stat;

use crate::infra::WrapResult;
use crate::sampling::group::inner::Inner;
use crate::sampling::record::Record;
use crate::sampling::Config;
use libc::pid_t;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use fixed::*;
pub use guard::*;
pub use stat::{MemberCount, SamplerGroupStat};

pub struct SamplerGroup {
    pid: pid_t,
    cpu: i32,
    mmap_pages: usize,
    inner: Arc<RwLock<Inner>>,
}

impl SamplerGroup {
    pub(crate) unsafe fn new(pid: pid_t, cpu: i32, mmap_pages: usize) -> Self {
        Self {
            pid,
            cpu,
            mmap_pages,
            inner: Arc::new(RwLock::new(Inner::new())),
        }
    }

    fn inner(&self) -> RwLockReadGuard<'_, Inner> {
        self.inner.read().unwrap()
    }

    fn inner_mut(&self) -> RwLockWriteGuard<'_, Inner> {
        self.inner.write().unwrap()
    }

    pub fn add_member(&mut self, cfg: &Config) -> io::Result<SamplerGuard> {
        let event_id = self
            .inner_mut()
            .add_member(self.pid, self.cpu, cfg, self.mmap_pages)?;
        SamplerGuard::new(event_id, self.inner.clone()).wrap_ok()
    }

    pub fn enable(self) -> io::Result<FixedSamplerGroup> {
        self.inner().enable()?;
        FixedSamplerGroup::new(self.inner).wrap_ok()
    }

    pub fn next_record(&self, guard: &SamplerGuard) -> Option<Record> {
        self.inner_mut().next_record(guard.event_id())
    }

    pub fn stat(&mut self) -> io::Result<SamplerGroupStat> {
        self.inner_mut().stat()
    }
}
