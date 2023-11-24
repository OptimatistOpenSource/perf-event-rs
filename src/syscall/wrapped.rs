use crate::syscall::ioctl;
use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;

pub fn ioctl_wrapped<A>(file: &File, request: impl Into<u64>, arg: Option<A>) -> io::Result<()> {
    let i32 = match arg {
        None => unsafe { ioctl(file.as_raw_fd() as libc::c_int, request.into(), 0) },
        Some(arg) => unsafe { ioctl(file.as_raw_fd() as libc::c_int, request.into(), arg) },
    };
    match i32 {
        -1 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}
