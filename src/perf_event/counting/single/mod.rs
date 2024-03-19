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

mod stat;
#[cfg(test)]
mod tests;

use crate::config;
use crate::config::{Cpu, Error, Process};
use crate::counting::single::stat::counter_stat;
use crate::counting::Config;
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open_wrapped};
pub use stat::CounterStat;
use std::fs::File;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd};

pub struct Counter {
    pub(crate) file: File,
}

impl Counter {
    pub fn new(process: &Process, cpu: &Cpu, cfg: &mut Config) -> config::Result<Self> {
        let (pid, cpu) = match (process.as_i32()?, cpu.as_i32()) {
            (-1, -1) => return Err(Error::InvalidProcessCpu),
            (pid, cpu) => (pid, cpu),
        };

        let perf_event_attr = cfg.as_raw_mut();
        // not inline `read_format` for readable
        #[rustfmt::skip]
        let read_format =
              PERF_FORMAT_TOTAL_TIME_ENABLED
            | PERF_FORMAT_TOTAL_TIME_RUNNING
            | PERF_FORMAT_ID;
        perf_event_attr.read_format = read_format as _;

        let fd = unsafe { perf_event_open_wrapped(perf_event_attr, pid, cpu, -1, 0) }
            .map_err(Error::SyscallFailed)?;
        let file = unsafe { File::from_raw_fd(fd) };

        Ok(Self { file })
    }

    pub fn enable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_ENABLE, None)
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_DISABLE, None)
    }

    pub fn reset(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_RESET, None)
    }

    pub fn set_output(&self, new: &File) -> io::Result<()> {
        let raw_fd = new.as_raw_fd() as i64;
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_SET_OUTPUT, Some(raw_fd))
    }

    pub fn ignore_output(&self) -> io::Result<()> {
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_SET_OUTPUT, Some(-1i64))
    }

    #[cfg(feature = "linux-3.12")]
    pub fn event_id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_ID, Some(&mut id))?;
        Ok(id)
    }

    pub fn stat(&mut self) -> io::Result<CounterStat> {
        counter_stat(self)
    }
}
