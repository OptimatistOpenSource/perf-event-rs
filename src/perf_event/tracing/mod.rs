mod builder;
mod config;
mod into_iter;
mod iter;
#[cfg(test)]
mod tests;

use crate::infra::{Vla, WrapResult};
use crate::sampling::record::Record;
use crate::sampling::Sampler;
use crate::syscall::bindings::*;
use crate::syscall::ioctl_wrapped;
use std::alloc::{alloc, Layout};
use std::io;

#[allow(unused_imports)]
pub use builder::*;
pub use config::*;
pub use into_iter::*;
pub use iter::*;

pub struct Tracer {
    pub(crate) sampler: Sampler,
}

impl Tracer {
    pub(crate) unsafe fn new(
        cfg: &Config,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
        mmap_pages: usize,
    ) -> io::Result<Self> {
        let sampler = Sampler::new_from_raw(cfg.as_raw(), pid, cpu, group_fd, flags, mmap_pages)?;
        Self { sampler }.wrap_ok()
    }

    pub fn enable(&self) -> io::Result<()> {
        self.sampler.enable()
    }

    pub fn disable(&self) -> io::Result<()> {
        self.sampler.disable()
    }

    pub fn pause(&self) -> io::Result<()> {
        self.sampler.pause()
    }

    pub fn resume(&self) -> io::Result<()> {
        self.sampler.resume()
    }

    // TODO: rm?
    pub fn refresh(&self, refresh: i32) -> io::Result<()> {
        self.sampler.refresh(refresh)
    }

    pub fn next_record(&mut self) -> Option<Record> {
        self.sampler.next_record()
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
    pub unsafe fn set_bpf(&self, bpf_fd: i32) -> io::Result<()> {
        ioctl_wrapped(&self.sampler.file, PERF_EVENT_IOCTL_SET_BPF, Some(bpf_fd))
    }

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

    pub fn update_cfg(&self, new: &Config) -> io::Result<()> {
        ioctl_wrapped(
            &self.sampler.file,
            PERF_EVENT_IOCTL_MODIFY_ATTRIBUTES,
            Some(new.as_raw()),
        )
    }
}
