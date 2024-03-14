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

use crate::config::Error;
#[cfg(feature = "linux-4.17")]
use crate::infra::Vla;
#[cfg(feature = "linux-4.17")]
use crate::infra::WrapResult;
use crate::sampling::record::Record;
use crate::sampling::{Sampler, SamplerStat};
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open_wrapped};
use memmap2::MmapOptions;
#[cfg(feature = "linux-4.17")]
use std::alloc::{alloc, Layout};
use std::fs::File;
use std::io;
use std::os::fd::FromRawFd;

use crate::config::{Cpu, Process};
use crate::tracing::Config;
#[allow(unused_imports)]
pub use into_iter::*;
#[allow(unused_imports)]
pub use iter::*;

pub struct Tracer {
    pub(crate) sampler: Sampler,
}

pub type TracerStat = SamplerStat;

impl Tracer {
    pub fn new(
        process: &Process,
        cpu: &Cpu,
        mmap_pages: usize,
        cfg: &Config,
    ) -> crate::config::Result<Self> {
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

        let sampler = Sampler {
            mmap,
            file,
            data_size: ((mmap_pages - 1) * page_size) as _,
            data_offset: page_size as _,
            sample_type: perf_event_attr.sample_type,
            sample_id_all: perf_event_attr.sample_id_all() > 0,
            regs_user_len: perf_event_attr.sample_regs_user.count_ones() as _,
            #[cfg(feature = "linux-3.19")]
            regs_intr_len: perf_event_attr.sample_regs_intr.count_ones() as _,
        };

        Ok(Self { sampler })
    }

    pub fn enable(&self) -> io::Result<()> {
        self.sampler.enable()
    }

    pub fn disable(&self) -> io::Result<()> {
        self.sampler.disable()
    }

    pub fn reset(&self) -> io::Result<()> {
        self.sampler.reset()
    }

    #[cfg(feature = "linux-4.7")]
    pub fn pause(&self) -> io::Result<()> {
        self.sampler.pause()
    }

    #[cfg(feature = "linux-4.7")]
    pub fn resume(&self) -> io::Result<()> {
        self.sampler.resume()
    }

    pub fn refresh(&self, refresh: i32) -> io::Result<()> {
        self.sampler.refresh(refresh)
    }

    pub fn next_record(&mut self) -> Option<Record> {
        self.sampler.next_record()
    }

    pub fn stat(&mut self) -> io::Result<TracerStat> {
        self.sampler.stat()
    }

    pub fn event_id(&self) -> io::Result<u64> {
        self.sampler.event_id()
    }

    /// # Safety
    /// The `ftrace_filter_ptr` argument should be a valid
    /// pointer to the desired ftrace filter.
    pub unsafe fn set_filter(&self, ftrace_filter_ptr: *const u8) -> io::Result<()> {
        ioctl_wrapped(
            &self.sampler.file,
            PERF_EVENT_IOCTL_SET_FILTER,
            Some(ftrace_filter_ptr),
        )
    }

    /// # Safety
    /// The `bpf_fd` argument should be a valid BPF program
    /// file descriptor that was created by a previous bpf(2) system call.
    #[cfg(feature = "linux-4.1")]
    pub unsafe fn set_bpf(&self, bpf_fd: i32) -> io::Result<()> {
        ioctl_wrapped(&self.sampler.file, PERF_EVENT_IOCTL_SET_BPF, Some(bpf_fd))
    }

    /// This allows querying which Berkeley Packet Filter (BPF)
    /// programs are attached to an existing kprobe tracepoint.
    #[cfg(feature = "linux-4.17")]
    pub fn query_bpf(&self, ids_len: u32) -> io::Result<Vec<u32>> {
        /*
        struct perf_event_query_bpf {
            __u32    ids_len;
            __u32    prog_cnt;
            __u32    ids[0];
        };
        */
        let layout = {
            let size = 4 + 4 + (ids_len * 4);
            Layout::from_size_align(size as _, 4).unwrap()
        };
        let ptr = unsafe { alloc(layout) } as *mut u32;
        unsafe { *ptr = ids_len };

        ioctl_wrapped(&self.sampler.file, PERF_EVENT_IOCTL_QUERY_BPF, Some(ptr))?;

        let vla: &Vla<u32, u32> = unsafe { Vla::from_ptr(ptr.add(1)) };
        vla.as_slice().to_vec().wrap_ok()
    }

    /// This allows modifying an existing event without the
    /// overhead of closing and reopening a new event.
    /// Currently this is supported only for breakpoint events.
    #[cfg(feature = "linux-4.17")]
    pub fn update_cfg(&self, new: &Config) -> io::Result<()> {
        ioctl_wrapped(
            &self.sampler.file,
            PERF_EVENT_IOCTL_MODIFY_ATTRIBUTES,
            Some(new.as_raw()),
        )
    }
}
