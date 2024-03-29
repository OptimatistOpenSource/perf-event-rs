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

use crate::infra::WrapResult;
use crate::sampling::group::inner::Inner;
use crate::sampling::record::Record;
use crate::sampling::Config;
use libc::pid_t;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::config;
use crate::config::{Cpu, Error, Process};
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
    pub fn new(process: &Process, cpu: &Cpu, mmap_pages: usize) -> config::Result<Self> {
        let (pid, cpu) = match (process.as_i32()?, cpu.as_i32()) {
            (-1, -1) => return Err(Error::InvalidProcessCpu),
            (pid, cpu) => (pid, cpu),
        };
        let inner = Arc::new(RwLock::new(Inner::new()));

        Self {
            pid,
            cpu,
            inner,
            mmap_pages,
        }
        .wrap_ok()
    }

    fn inner(&self) -> RwLockReadGuard<'_, Inner> {
        self.inner.read().unwrap()
    }

    fn inner_mut(&self) -> RwLockWriteGuard<'_, Inner> {
        self.inner.write().unwrap()
    }

    pub fn add_member(&mut self, cfg: &Config) -> io::Result<SamplerGuard> {
        let event_id =
            self.inner_mut()
                .add_member(self.pid, self.cpu, self.mmap_pages, cfg.as_raw())?;
        SamplerGuard::new(event_id, self.inner.clone()).wrap_ok()
    }

    pub fn enable(self) -> io::Result<FixedSamplerGroup> {
        self.inner().enable()?;
        self.into_fixed()
    }

    pub fn next_record(&self, guard: &SamplerGuard) -> Option<Record> {
        self.inner_mut().next_record(guard.event_id())
    }

    pub fn stat(&mut self) -> io::Result<SamplerGroupStat> {
        self.inner_mut().stat()
    }

    pub fn into_fixed(self) -> io::Result<FixedSamplerGroup> {
        FixedSamplerGroup::new(self.inner).wrap_ok()
    }
}
