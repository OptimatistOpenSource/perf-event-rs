mod fixed;
mod guard;
mod inner;

use crate::infra::WrapResult;
use crate::sampling::group::fixed::FixedSamplingGroup;
use crate::sampling::group::guard::SamplingGuard;
use crate::sampling::group::inner::Inner;
use crate::sampling::record::Record;
use crate::sampling::Attr;
use libc::pid_t;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct SamplingGroup {
    pid: pid_t,
    cpu: i32,
    inner: Arc<RwLock<Inner>>,
}

impl SamplingGroup {
    pub(crate) unsafe fn new(pid: pid_t, cpu: i32) -> Self {
        Self {
            pid,
            cpu,
            inner: Arc::new(RwLock::new(Inner::new())),
        }
    }

    fn inner(&self) -> RwLockReadGuard<'_, Inner> {
        self.inner.read().unwrap()
    }

    fn inner_mut(&self) -> RwLockWriteGuard<'_, Inner> {
        self.inner.write().unwrap()
    }

    pub fn add_member(&mut self, attr: &Attr) -> io::Result<SamplingGuard> {
        let event_id = self.inner_mut().add_member(self.pid, self.cpu, attr)?;
        SamplingGuard::new(event_id, self.inner.clone()).wrap_ok()
    }

    pub fn enable(self) -> io::Result<FixedSamplingGroup> {
        self.inner().enable()?;
        FixedSamplingGroup::new(self.inner).wrap_ok()
    }

    pub fn next_record(&self, guard: &SamplingGuard) -> Option<Record> {
        self.inner_mut().next_record(guard.event_id())
    }
}
