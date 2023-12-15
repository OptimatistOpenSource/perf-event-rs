mod attr;
mod builder;
mod into_iter;
mod iter;
mod tests;

use crate::infra::{Vla, WrapResult};
use crate::sampling::record::Record;
use crate::sampling::Sampling;
use crate::syscall::bindings::*;
use crate::syscall::ioctl_wrapped;
use std::alloc::{alloc, Layout};
use std::io;

pub use attr::*;
pub use builder::*;
pub use into_iter::*;
pub use iter::*;

pub struct Tracing {
    pub(crate) sampling: Sampling,
}

impl Tracing {
    pub(crate) unsafe fn new(
        attr: &Attr,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
        mmap_pages: usize,
    ) -> io::Result<Self> {
        let sampling =
            Sampling::new_from_raw(attr.as_raw(), pid, cpu, group_fd, flags, mmap_pages)?;
        Self { sampling }.wrap_ok()
    }

    pub fn enable(&self) -> io::Result<()> {
        self.sampling.enable()
    }

    pub fn disable(&self) -> io::Result<()> {
        self.sampling.disable()
    }

    pub fn pause(&self) -> io::Result<()> {
        self.sampling.pause()
    }

    pub fn resume(&self) -> io::Result<()> {
        self.sampling.resume()
    }

    // TODO: rm?
    pub fn refresh(&self, refresh: i32) -> io::Result<()> {
        self.sampling.refresh(refresh)
    }

    pub fn next_record(&mut self) -> Option<Record> {
        self.sampling.next_record()
    }

    pub fn event_id(&self) -> io::Result<u64> {
        self.sampling.event_id()
    }

    /// # Safety
    /// The `ftrace_filter_ptr` argument should be a valid
    /// pointer to the desired ftrace filter.
    pub unsafe fn set_filter(&self, ftrace_filter_ptr: *const u8) -> io::Result<()> {
        ioctl_wrapped(
            &self.sampling.file,
            PERF_EVENT_IOCTL_SET_FILTER,
            Some(ftrace_filter_ptr),
        )
    }

    /// # Safety
    /// The `bpf_fd` argument should be a valid BPF program
    /// file descriptor that was created by a previous bpf(2) system call.
    pub unsafe fn set_bpf(&self, bpf_fd: i32) -> io::Result<()> {
        ioctl_wrapped(&self.sampling.file, PERF_EVENT_IOCTL_SET_BPF, Some(bpf_fd))
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

        ioctl_wrapped(&self.sampling.file, PERF_EVENT_IOCTL_QUERY_BPF, Some(ptr))?;

        let vla: &Vla<u32, u32> = unsafe { Vla::from_ptr(ptr.add(1)).as_ref() }.unwrap();
        vla.as_slice().to_vec().wrap_ok()
    }

    pub fn update_attr(&self, new: &Attr) -> io::Result<()> {
        ioctl_wrapped(
            &self.sampling.file,
            PERF_EVENT_IOCTL_MODIFY_ATTRIBUTES,
            Some(new.as_raw()),
        )
    }
}
