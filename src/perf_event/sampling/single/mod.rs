mod into_iter;
mod iter;
mod next_record;

use crate::infra::WrapResult;
use crate::perf_event::RawAttr;
use crate::sampling::record::*;
use crate::sampling::single::next_record::next_record;
use crate::sampling::Attr;
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open};
use memmap::{MmapMut, MmapOptions};
use std::fs::File;
use std::io;
use std::os::fd::FromRawFd;

pub use into_iter::*;
pub use iter::*;

pub struct Sampling {
    pub(crate) mmap: MmapMut,
    pub(crate) file: File,

    /// The size of 2^n part of 1 + 2^n mmap pages,
    /// i.e. `perf_event_mmap_page.data_size`\
    /// Cached here for performance
    pub(crate) data_size: u64,

    pub(crate) sample_type: u64,
    pub(crate) sample_id_all: bool,

    pub(crate) regs_user_len: usize,
    pub(crate) regs_intr_len: usize,
}

impl Sampling {
    pub(crate) unsafe fn new_from_raw(
        raw_attr: &RawAttr,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
        mmap_pages: usize,
    ) -> io::Result<Self> {
        let i32 = unsafe { perf_event_open(raw_attr, pid, cpu, group_fd, flags) };
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

                let data_size = (mmap.as_ptr() as *const perf_event_mmap_page)
                    .as_ref()
                    .unwrap()
                    .data_size;

                Self {
                    mmap,
                    file,
                    data_size,
                    sample_type: raw_attr.sample_type,
                    sample_id_all: raw_attr.sample_id_all() > 0,
                    regs_user_len: raw_attr.sample_regs_user.count_ones() as _,
                    regs_intr_len: raw_attr.sample_regs_intr.count_ones() as _,
                }
            }
            .wrap_ok(),
        }
    }

    pub(crate) unsafe fn new(
        attr: &Attr,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
        mmap_pages: usize,
    ) -> io::Result<Self> {
        Self::new_from_raw(attr.as_raw(), pid, cpu, group_fd, flags, mmap_pages)
    }

    pub fn enable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_ENABLE, None)
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_DISABLE, None)
    }

    pub fn pause(&self) -> io::Result<()> {
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_PAUSE_OUTPUT, Some(1i32))
    }

    pub fn resume(&self) -> io::Result<()> {
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_PAUSE_OUTPUT, Some(0i32))
    }

    pub fn refresh(&self, refresh: i32) -> io::Result<()> {
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_REFRESH, Some(refresh))
    }

    pub fn next_record(&mut self) -> Option<Record> {
        next_record(self)
    }

    pub fn event_id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_ID, Some(&mut id))?;
        Ok(id)
    }
}
