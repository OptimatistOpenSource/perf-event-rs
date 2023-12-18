use crate::counting::Config;
use crate::infra::{SizedExt, WrapResult};
use crate::syscall;
use crate::syscall::bindings::{read_format_body, read_format_header};
use crate::syscall::{ioctl_wrapped, perf_event_open};
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd};

pub struct Counting {
    pub(crate) file: File,
}

#[derive(Debug, Clone)]
pub struct CountingResult {
    pub event_count: u64,
    pub time_enabled: u64,
    pub time_running: u64,
}

impl Counting {
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

    pub fn result(&mut self) -> io::Result<CountingResult> {
        #[repr(C)]
        #[allow(non_camel_case_types)]
        struct read_format {
            header: read_format_header,
            body: read_format_body, // This group has only one member
        }

        let mut buf = unsafe { <[u8; std::mem::size_of::<read_format>()]>::uninit() };
        self.file.read_exact(&mut buf)?;

        let read_format = unsafe { &*(buf.as_ptr() as *const read_format) };
        CountingResult {
            event_count: read_format.body.event_count,
            time_enabled: read_format.header.time_enabled,
            time_running: read_format.header.time_running,
        }
        .wrap_ok()
    }

    pub fn enable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, syscall::bindings::PERF_EVENT_IOCTL_ENABLE, None)
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(
            &self.file,
            syscall::bindings::PERF_EVENT_IOCTL_DISABLE,
            None,
        )
    }

    pub fn reset_count(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, syscall::bindings::PERF_EVENT_IOCTL_RESET, None)
    }

    pub fn update_period(&self, new: u64) -> io::Result<()> {
        ioctl_wrapped(
            &self.file,
            syscall::bindings::PERF_EVENT_IOCTL_PERIOD,
            Some(&new),
        )
    }

    pub fn set_output(&self, new: &File) -> io::Result<()> {
        let raw_fd = new.as_raw_fd() as i64;
        ioctl_wrapped(
            &self.file,
            syscall::bindings::PERF_EVENT_IOCTL_SET_OUTPUT,
            Some(raw_fd),
        )
    }

    pub fn ignore_output(&self) -> io::Result<()> {
        ioctl_wrapped(
            &self.file,
            syscall::bindings::PERF_EVENT_IOCTL_SET_OUTPUT,
            Some(-1i64),
        )
    }

    pub fn event_id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        ioctl_wrapped(
            &self.file,
            syscall::bindings::PERF_EVENT_IOCTL_ID,
            Some(&mut id),
        )?;
        Ok(id)
    }
}
