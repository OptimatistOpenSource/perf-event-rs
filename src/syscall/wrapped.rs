use crate::syscall::bindings::perf_event_attr;
use crate::syscall::{ioctl, perf_event_open};
use std::ffi::c_int;
use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;

pub fn ioctl_wrapped<A>(file: &File, request: impl Into<u64>, arg: Option<A>) -> io::Result<()> {
    let i32 = match arg {
        None => unsafe { ioctl(file.as_raw_fd() as _, request.into(), 0) },
        Some(arg) => unsafe { ioctl(file.as_raw_fd() as _, request.into(), arg) },
    };
    match i32 {
        -1 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}

pub unsafe fn perf_event_open_wrapped(
    raw_attr: &perf_event_attr,
    pid: i32,
    cpu: i32,
    group_fd: i32,
    flags: u64,
) -> io::Result<c_int> {
    let i32 = perf_event_open(raw_attr, pid, cpu, group_fd, flags);
    match i32 {
        -1 => Err(io::Error::last_os_error()),
        fd => Ok(fd),
    }
}
