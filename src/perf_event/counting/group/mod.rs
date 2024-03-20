// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of perf-event-rs.
//
// Perf-event-rs is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Perf-event-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

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
use crate::syscall::bindings::*;
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

    pub fn add_member(&mut self, cfg: &mut Config) -> io::Result<CounterGuard> {
        let perf_event_attr = cfg.as_raw_mut();
        // not inline `read_format` for readable
        #[rustfmt::skip]
        let read_format =
              PERF_FORMAT_TOTAL_TIME_ENABLED
            | PERF_FORMAT_TOTAL_TIME_RUNNING
            | PERF_FORMAT_ID
            | PERF_FORMAT_GROUP;
        perf_event_attr.read_format = read_format as _;

        let event_id = self
            .inner_mut()
            .add_member(self.cpu, self.pid, perf_event_attr)?;
        CounterGuard::new(event_id, self.inner.clone()).wrap_ok()
    }

    pub fn enable(self) -> io::Result<FixedCounterGroup> {
        self.inner_mut().enable()?;
        self.into_fixed()
    }

    pub fn stat(&mut self) -> io::Result<CounterGroupStat> {
        self.inner_mut().stat()
    }

    pub fn into_fixed(self) -> io::Result<FixedCounterGroup> {
        FixedCounterGroup::new(self.inner).wrap_ok()
    }
}
