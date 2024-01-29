mod stat;
#[cfg(test)]
mod tests;

use crate::counting::single::stat::counter_stat;
use crate::counting::Config;
use crate::infra::WrapResult;
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open};
pub use stat::CounterStat;
use std::fs::File;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd};

pub struct Counter {
    pub(crate) file: File,
}

impl Counter {
    pub(crate) unsafe fn new(
        cfg: &Config,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
    ) -> io::Result<Self> {
        let i32 = unsafe { perf_event_open(cfg.as_raw(), pid, cpu, group_fd, flags) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            fd => Self {
                file: File::from_raw_fd(fd),
            }
            .wrap_ok(),
        }
    }

    pub fn enable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_ENABLE, None)
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_DISABLE, None)
    }

    pub fn reset_count(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, PERF_EVENT_IOCTL_RESET, None)
    }

    pub fn set_output(&self, new: &File) -> io::Result<()> {
        let raw_fd = new.as_raw_fd() as i64;
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_SET_OUTPUT, Some(raw_fd))
    }

    pub fn ignore_output(&self) -> io::Result<()> {
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_SET_OUTPUT, Some(-1i64))
    }

    #[cfg(feature = "linux-3.12")]
    pub fn event_id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        ioctl_wrapped(&self.file, PERF_EVENT_IOCTL_ID, Some(&mut id))?;
        Ok(id)
    }

    pub fn stat(&mut self) -> io::Result<CounterStat> {
        counter_stat(self)
    }
}
