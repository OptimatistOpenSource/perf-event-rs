mod attr;
mod builder;
mod record;
#[cfg(test)]
mod tests;

use crate::infra::{ArrayExt, VecExt, WrapResult};
use crate::sampling::record::sample;
use crate::syscall;
use crate::syscall::bindings::{
    perf_event_header, perf_event_mmap_page, perf_event_type_PERF_RECORD_SAMPLE,
};
use crate::syscall::{ioctl_wrapped, perf_event_open};
pub use attr::*;
pub use builder::*;
use memmap::{MmapMut, MmapOptions};
use std::fs::File;
use std::os::fd::FromRawFd;
use std::{io, slice};

pub struct Sampling {
    pub(crate) mmap: MmapMut,
    pub(crate) file: File,
}

// TODO
impl Sampling {
    pub(crate) unsafe fn new(
        attr: &Attr,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
    ) -> io::Result<Self> {
        let i32 = unsafe { perf_event_open(attr.as_raw(), pid, cpu, group_fd, flags) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            fd => {
                let file = File::from_raw_fd(fd);
                let pages = 1 + (1 << 16); // TODO
                let mmap = unsafe {
                    MmapOptions::new()
                        .len(page_size::get() * pages)
                        .map_mut(&file)
                }
                .unwrap();

                Self { mmap, file }
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
