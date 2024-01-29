mod into_iter;
mod iter;
mod next_record;
mod stat;

use crate::infra::WrapResult;
use crate::perf_event::RawAttr;
use crate::sampling::record::*;
use crate::sampling::single::next_record::next_record;
use crate::sampling::Config;
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open};
use memmap2::{MmapMut, MmapOptions};
use std::fs::File;
use std::io;
use std::os::fd::FromRawFd;

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

                let page_size = page_size::get();

                Self {
                    mmap,
                    file,
                    data_size: ((mmap_pages - 1) * page_size) as _,
                    data_offset: page_size as _,
                    sample_type: raw_attr.sample_type,
                    sample_id_all: raw_attr.sample_id_all() > 0,
                    regs_user_len: raw_attr.sample_regs_user.count_ones() as _,
                    #[cfg(feature = "linux-3.19")]
                    regs_intr_len: raw_attr.sample_regs_intr.count_ones() as _,
                }
            }
            .wrap_ok(),
        }
    }

    pub(crate) unsafe fn new(
        cfg: &Config,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
        mmap_pages: usize,
    ) -> io::Result<Self> {
        Self::new_from_raw(cfg.as_raw(), pid, cpu, group_fd, flags, mmap_pages)
    }

    pub fn enable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_ENABLE, None)
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_DISABLE, None)
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
