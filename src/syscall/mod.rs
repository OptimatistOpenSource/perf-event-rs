pub mod bindings;
mod wrapped;

use libc::{c_int, c_ulong, pid_t};
pub use wrapped::*;

/// # Safety
/// The arguments must be correct for this syscall
pub unsafe fn perf_event_open(
    attr: &bindings::perf_event_attr,
    pid: pid_t,
    cpu: c_int,      //i32
    group_fd: c_int, //i32
    flags: c_ulong,  //u64
) -> c_int {
    libc::syscall(
        bindings::__NR_perf_event_open as _,
        attr,
        pid,
        cpu,
        group_fd,
        flags,
    ) as _
}

/// # Safety
/// The arguments must be correct for this syscall
pub unsafe fn ioctl<A>(
    fd: c_int,        //i32
    request: c_ulong, //u64
    arg: A,
) -> c_int {
    libc::ioctl(fd, request, arg)
}
