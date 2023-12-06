mod next_record;

use crate::infra::WrapResult;
use crate::sampling::record::*;
use crate::sampling::single::next_record::next_record;
use crate::sampling::Attr;
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open};
use memmap::{MmapMut, MmapOptions};
use std::fs::File;
use std::io;
use std::os::fd::FromRawFd;

pub struct Sampling {
    pub(crate) mmap: MmapMut,
    pub(crate) file: File,
    pub(crate) is_sample_callchain: bool,
    pub(crate) user_regs_len: usize,
    pub(crate) intr_regs_len: usize,
}

impl Sampling {
    pub(crate) unsafe fn new(
        attr: &Attr,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
        mmap_pages: usize,
    ) -> io::Result<Self> {
        let i32 = unsafe { perf_event_open(attr.as_raw(), pid, cpu, group_fd, flags) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            fd => {
                let file = File::from_raw_fd(fd);
                let mmap = unsafe {
                    MmapOptions::new()
                        .len(page_size::get() * mmap_pages)
                        .map_mut(&file)
                }
                .unwrap();

                let is_sample_callchain = attr.as_raw().sample_max_stack > 0;
                let user_regs_len = attr.as_raw().sample_regs_user.count_ones() as _;
                let intr_regs_len = attr.as_raw().sample_regs_intr.count_ones() as _;
                Self {
                    mmap,
                    file,
                    is_sample_callchain,
                    user_regs_len,
                    intr_regs_len,
                }
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

    pub fn next_record(&mut self) -> Option<Record> {
        next_record(self)
    }

    pub fn event_id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        ioctl_wrapped(&self.file, perf_event_ioctls_ID, Some(&mut id))?;
        Ok(id)
    }
}

impl Iterator for Sampling {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_record()
    }
}
