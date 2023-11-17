mod builder;
mod counting;

use crate::syscall;
use crate::syscall::bindings::perf_event_ioctls;
use crate::syscall::ioctl;
use std::fs::File;
use std::io;
use std::os::fd::{AsRawFd, RawFd};

pub use builder::*;
pub use counting::attr::*;
pub use counting::hw_event::*;
pub use counting::sw_event::*;

pub struct PerfEvent {
    // TODO
    raw_fd: RawFd,
}

impl PerfEvent {
    fn perf_event_ioctl(&self, op: perf_event_ioctls) -> io::Result<()> {
        let i32 = unsafe { ioctl(self.raw_fd as libc::c_int, op as libc::c_ulong, 0) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            _ => Ok(()),
        }
    }

    fn perf_event_ioctl_with_arg<A>(&self, op: perf_event_ioctls, arg: A) -> io::Result<()> {
        let i32 = unsafe { ioctl(self.raw_fd as libc::c_int, op as libc::c_ulong, arg) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            _ => Ok(()),
        }
    }

    pub fn enable(&self) -> io::Result<()> {
        self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_ENABLE)
    }

    pub fn enable_group(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(
            syscall::bindings::perf_event_ioctls_ENABLE,
            syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP,
        )
    }

    pub fn disable(&self) -> io::Result<()> {
        self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_DISABLE)
    }

    pub fn disable_group(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(
            syscall::bindings::perf_event_ioctls_DISABLE,
            syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP,
        )
    }

    pub fn refresh(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_REFRESH)
        todo!()
    }

    pub fn reset_count(&self) -> io::Result<()> {
        self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_RESET)
    }

    pub fn reset_count_group(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(
            syscall::bindings::perf_event_ioctls_RESET,
            syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP,
        )
    }

    pub fn update_period(&self, new: u64) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PERIOD, &new)
    }

    pub fn set_output(&self, new: File) -> io::Result<()> {
        let raw_fd = new.as_raw_fd() as i64;
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_SET_OUTPUT, raw_fd)
    }

    pub fn ignore_output(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_SET_OUTPUT, -1i64)
    }

    pub fn set_filter(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_FILTER)
        todo!()
    }

    pub fn id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_ID, &mut id)?;
        Ok(id)
    }

    pub fn set_bpf(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_BPF)
        todo!()
    }

    pub fn pause_output(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 1i32)
    }

    pub fn resume_output(&self) -> io::Result<()> {
        self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 0i32)
    }

    pub fn query_bpf(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_QUERY_BPF)
        todo!()
    }

    pub fn modify_attributes(&self) -> io::Result<()> {
        //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_MODIFY_ATTRIBUTES)
        todo!()
    }
}
