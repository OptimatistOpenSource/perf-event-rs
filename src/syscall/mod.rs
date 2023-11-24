#[allow(clippy::useless_transmute)]
#[allow(clippy::unnecessary_cast)]
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod bindings; // TODO: bindings may differ between kernel versions
mod bindings_impl;
mod wrapped;

use libc::{c_int, c_ulong, pid_t};
pub use wrapped::*;

/// # Safety
/// The arguments must be correct for this syscall
pub unsafe fn perf_event_open(
    attr: *const bindings::perf_event_attr, // TODO: validating references with lifetimes
    pid: pid_t,
    cpu: c_int,      //i32
    group_fd: c_int, //i32
    flags: c_ulong,  //u64
) -> c_int {
    libc::syscall(
        bindings::__NR_perf_event_open as libc::c_long,
        attr,
        pid,
        cpu,
        group_fd,
        flags,
    ) as c_int
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
