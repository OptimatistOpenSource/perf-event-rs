// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of perf-event-rs.
//
// Perf-event-rs is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License 
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// 
// Perf-event-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not, 
// see <https://www.gnu.org/licenses/>.

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
    libc::ioctl(fd, request as _, arg)
}
