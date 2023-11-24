mod attr;
mod builder;
mod record;
#[cfg(test)]
mod tests;

use crate::infra::WrapResult;
use crate::perf_event::RawAttr;
use crate::sampling::record::sample;
use crate::syscall;
use crate::syscall::bindings::perf_event_mmap_page;
use crate::syscall::{ioctl_wrapped, perf_event_open};
pub use attr::*;
pub use builder::*;
use memmap::MmapOptions;
use std::fs::File;
use std::io;
use std::os::fd::FromRawFd;

pub struct Sampling {
    // TODO
    #[allow(dead_code)]
    pub(crate) raw_attr: Box<RawAttr>,
    pub(crate) file: File,
}

// TODO
impl Sampling {
    pub(crate) unsafe fn new(
        attr: Attr,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
    ) -> io::Result<Self> {
        let raw_attr = Box::new(attr.into_raw());
        let i32 = unsafe { perf_event_open(&*raw_attr as *const _, pid, cpu, group_fd, flags) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            fd => Self {
                raw_attr,
                file: File::from_raw_fd(fd),
            }
            .wrap_ok(),
        }
    }

    pub fn enable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(
            &self.file,
            syscall::bindings::perf_event_ioctls_ENABLE,
            None,
        )
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(
            &self.file,
            syscall::bindings::perf_event_ioctls_DISABLE,
            None,
        )
    }

    pub fn next_sample() {

    }
}
