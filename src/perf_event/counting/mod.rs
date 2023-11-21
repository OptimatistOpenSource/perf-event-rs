use crate::syscall;
use crate::syscall::bindings::perf_event_attr;
use crate::syscall::{ioctl, perf_event_open};
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd};

mod attr;
mod builder;
mod event;

#[cfg(test)]
mod tests;

use crate::infra::result::WrapResult;
pub use attr::*;
#[allow(unused_imports)]
pub use builder::*;
pub use event::*;

fn ioctl_wrapped<A>(file: &File, request: impl Into<u64>, arg: Option<A>) -> io::Result<()> {
    let i32 = match arg {
        None => unsafe { ioctl(file.as_raw_fd() as libc::c_int, request.into(), 0) },
        Some(arg) => unsafe { ioctl(file.as_raw_fd() as libc::c_int, request.into(), arg) },
    };
    match i32 {
        -1 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}

pub struct Counting {
    // TODO
    #[allow(dead_code)]
    raw_attr: Box<perf_event_attr>,
    file: File,
}

#[repr(C)]
#[derive(Debug)]
pub struct CountingResult {
    pub event_count: u64,
    pub time_enabled: u64,
    pub time_running: u64,
    pub event_id: u64,
    #[cfg(feature = "kernel-6.0")]
    pub event_lost: u64,
}

impl Counting {
    unsafe fn new(attr: Attr, pid: i32, cpu: i32, group_fd: i32, flags: u64) -> io::Result<Self> {
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

    pub fn get_result(&mut self) -> io::Result<CountingResult> {
        let mut buf = [0_u8; std::mem::size_of::<CountingResult>()];

        match self.file.read_exact(&mut buf) {
            Ok(()) => {
                let read_format_ptr = buf.as_ptr() as *const CountingResult;
                unsafe { read_format_ptr.read() }.wrap_ok()
            }
            Err(e) => Err(e),
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

    /*
        // TODO
        pub fn refresh(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_REFRESH)
            todo!()
        }
    */

    pub fn reset_count(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(&self.file, syscall::bindings::perf_event_ioctls_RESET, None)
    }

    pub fn update_period(&self, new: u64) -> io::Result<()> {
        ioctl_wrapped(
            &self.file,
            syscall::bindings::perf_event_ioctls_PERIOD,
            Some(&new),
        )
    }

    pub fn set_output(&self, new: &File) -> io::Result<()> {
        let raw_fd = new.as_raw_fd() as i64;
        ioctl_wrapped(
            &self.file,
            syscall::bindings::perf_event_ioctls_SET_OUTPUT,
            Some(raw_fd),
        )
    }

    pub fn ignore_output(&self) -> io::Result<()> {
        ioctl_wrapped(
            &self.file,
            syscall::bindings::perf_event_ioctls_SET_OUTPUT,
            Some(-1i64),
        )
    }

    /*
        // TODO
        pub fn set_filter(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_FILTER)
            todo!()
        }
    */

    pub fn id(&self) -> io::Result<u64> {
        let mut id = 0_u64;
        ioctl_wrapped(
            &self.file,
            syscall::bindings::perf_event_ioctls_ID,
            Some(&mut id),
        )?;
        Ok(id)
    }

    /*
        // TODO
        pub fn set_bpf(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_SET_BPF)
            todo!()
        }

        // TODO: sampling mode only
        pub fn pause_output(&self) -> io::Result<()> {
            self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 1i32)
        }

        // TODO: sampling mode only
        pub fn resume_output(&self) -> io::Result<()> {
            self.perf_event_ioctl_with_arg(syscall::bindings::perf_event_ioctls_PAUSE_OUTPUT, 0i32)
        }

        // TODO
        pub fn query_bpf(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_QUERY_BPF)
            todo!()
        }

        // TODO
        pub fn modify_attributes(&self) -> io::Result<()> {
            //self.perf_event_ioctl(syscall::bindings::perf_event_ioctls_MODIFY_ATTRIBUTES)
            todo!()
        }
    */
}

pub struct CountingGroup {
    leader: Counting,
}

impl CountingGroup {
    pub fn enable(&self) -> io::Result<()> {
        ioctl_wrapped(
            &self.leader.file,
            syscall::bindings::perf_event_ioctls_ENABLE,
            Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
        )
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped(
            &self.leader.file,
            syscall::bindings::perf_event_ioctls_DISABLE,
            Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
        )
    }

    pub fn reset_count(&self) -> io::Result<()> {
        ioctl_wrapped(
            &self.leader.file,
            syscall::bindings::perf_event_ioctls_RESET,
            Some(syscall::bindings::perf_event_ioc_flags_PERF_IOC_FLAG_GROUP),
        )
    }
}
