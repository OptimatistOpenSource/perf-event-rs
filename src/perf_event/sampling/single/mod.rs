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

mod into_iter;
mod iter;
mod next_record;
mod stat;
#[cfg(test)]
mod tests;

use crate::config;
use crate::infra::WrapResult;
use crate::sampling::record::*;
use crate::sampling::single::next_record::next_record;
use crate::sampling::Config;
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open_wrapped};
use memmap2::{MmapMut, MmapOptions};
use std::fs::File;
use std::io;
use std::os::fd::FromRawFd;

use crate::config::{Cpu, Error, Process};
use crate::sampling::single::stat::sampler_stat;
pub use into_iter::*;
pub use iter::*;
pub use stat::SamplerStat;

pub struct Sampler {
    pub(crate) mmap: MmapMut,
    pub(crate) file: File,

    /*
    `data_size` and `data_offset` are saved here for performance
    and compatibility reasons (before Linux 4.1)
    See: https://github.com/torvalds/linux/commit/e8c6deac69629c0cb97c3d3272f8631ef17f8f0f
    */
    /// Size of 2^n part of 1 + 2^n mmap pages,
    /// i.e. `perf_event_mmap_page.data_size`
    pub(crate) data_size: u64,
    /// Size of one page,
    /// i.e. `perf_event_mmap_page.data_offset`
    pub(crate) data_offset: u64,

    pub(crate) sample_type: u64,
    pub(crate) sample_id_all: bool,

    pub(crate) regs_user_len: usize,
    #[cfg(feature = "linux-3.19")]
    pub(crate) regs_intr_len: usize,
}

impl Sampler {
    pub fn new(
        process: &Process,
        cpu: &Cpu,
        mmap_pages: usize,
        cfg: &Config,
    ) -> config::Result<Self> {
        let (pid, cpu) = match (process.as_i32()?, cpu.as_i32()) {
            (-1, -1) => return Err(Error::InvalidProcessCpu),
            (pid, cpu) => (pid, cpu),
        };
        let perf_event_attr = cfg.as_raw();

        let fd = unsafe { perf_event_open_wrapped(perf_event_attr, pid, cpu, -1, 0) }
            .map_err(Error::SyscallFailed)?;
        let file = unsafe { File::from_raw_fd(fd) };

        let mmap = unsafe {
            MmapOptions::new()
                .len(page_size::get() * mmap_pages)
                .map_mut(&file)
        }
        .unwrap();

        let page_size = page_size::get();

        Self {
            mmap,
            file,
            data_size: ((mmap_pages - 1) * page_size) as _,
            data_offset: page_size as _,
            sample_type: perf_event_attr.sample_type,
            sample_id_all: perf_event_attr.sample_id_all() > 0,
            regs_user_len: perf_event_attr.sample_regs_user.count_ones() as _,
            #[cfg(feature = "linux-3.19")]
            regs_intr_len: perf_event_attr.sample_regs_intr.count_ones() as _,
        }
        .wrap_ok()
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

    #[cfg(feature = "linux-4.7")]
    pub fn pause(&self) -> io::Result<()> {
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_PAUSE_OUTPUT, Some(1i32))
    }

    #[cfg(feature = "linux-4.7")]
    pub fn resume(&self) -> io::Result<()> {
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_PAUSE_OUTPUT, Some(0i32))
    }

    pub fn refresh(&self, refresh: i32) -> io::Result<()> {
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_REFRESH, Some(refresh))
    }

    pub fn update_period(&self, new: u64) -> io::Result<()> {
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_PERIOD, Some(&new))
    }

    pub fn next_record(&mut self) -> Option<Record> {
        next_record(self)
    }

    #[cfg(feature = "linux-3.12")]
    pub fn event_id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_ID, Some(&mut id))?;
        Ok(id)
    }

    pub fn stat(&mut self) -> io::Result<SamplerStat> {
        sampler_stat(self)
    }
}
