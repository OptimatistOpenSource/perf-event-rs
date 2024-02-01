mod fixed;
mod guard;
mod inner;
mod stat;
#[cfg(test)]
mod tests;

use crate::counting::Config;
use crate::infra::WrapResult;
use libc::pid_t;
pub use stat::*;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::config;
use crate::config::{Cpu, Process};
use crate::counting::group::inner::Inner;
pub use fixed::*;
#[allow(unused_imports)]
pub use guard::*;
#[allow(unused_imports)]
pub use stat::CounterGroupStat;

pub struct CounterGroup {
    pid: pid_t,
    cpu: i32,
    inner: Arc<RwLock<Inner>>,
}

impl CounterGroup {
    pub fn new(process: &Process, cpu: &Cpu) -> config::Result<Self> {
        let (pid, cpu) = match (process.as_i32()?, cpu.as_i32()) {
            (-1, -1) => return Err(config::Error::InvalidProcessCpu),
            (pid, cpu) => (pid, cpu),
        };
        let inner = Arc::new(RwLock::new(Inner::new()));

        Ok(Self { pid, cpu, inner })
    }

    #[allow(dead_code)]
    fn inner(&self) -> RwLockReadGuard<'_, Inner> {
        self.inner.read().unwrap()
    }

    fn inner_mut(&self) -> RwLockWriteGuard<'_, Inner> {
        self.inner.write().unwrap()
    }

    pub fn add_member(&mut self, cfg: &Config) -> io::Result<CounterGuard> {
        let event_id = self
            .inner_mut()
            .add_member(self.cpu, self.pid, cfg.as_raw())?;
        CounterGuard::new(event_id, self.inner.clone()).wrap_ok()
    }

    pub fn enable(self) -> io::Result<FixedCounterGroup> {
        self.inner_mut().enable()?;
        FixedCounterGroup::new(self.inner).wrap_ok()
    }

    pub fn stat(&mut self) -> io::Result<CounterGroupStat> {
        self.inner_mut().stat()
    }
}
