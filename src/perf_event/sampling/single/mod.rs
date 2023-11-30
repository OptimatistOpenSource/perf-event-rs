mod next_sample;

use crate::infra::WrapResult;
use crate::sampling::record::*;
use crate::sampling::single::next_sample::next_sample;
use crate::sampling::Attr;
use crate::syscall;
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open};
use memmap::{MmapMut, MmapOptions};
use std::fs::File;
use std::io;
use std::os::fd::FromRawFd;

pub struct Sampling {
    pub(crate) mmap: MmapMut,
    pub(crate) file: File,
}

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
        ioctl_wrapped::<()>(&self.file, perf_event_ioctls_ENABLE, None)
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, perf_event_ioctls_DISABLE, None)
    }

    pub fn pause(&self) -> io::Result<()> {
        ioctl_wrapped(&self.file, perf_event_ioctls_PAUSE_OUTPUT, Some(1i32))
    }

    pub fn resume(&self) -> io::Result<()> {
        ioctl_wrapped(&self.file, perf_event_ioctls_PAUSE_OUTPUT, Some(0i32))
    }

    pub fn next_sample(&mut self) -> Option<Record> {
        next_sample(self)
    }

    pub fn event_id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        ioctl_wrapped(
            &self.file,
            syscall::bindings::perf_event_ioctls_ID,
            Some(&mut id),
        )?;
        Ok(id)
    }
}

impl Iterator for Sampling {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_sample()
    }
}
