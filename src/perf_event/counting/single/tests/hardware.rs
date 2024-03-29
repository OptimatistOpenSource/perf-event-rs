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

use crate::counting::single::tests::test_single;
use crate::test::cpu_workload;
use crate::{Event, HardwareEvent};

#[test]
fn test_cpu_cycles() {
    let ev = HardwareEvent::CpuCycles;
    let mut workload = cpu_workload;

    test_single(&Event::from(ev), &mut workload);
}

#[test]
#[cfg(target_arch = "aarch64")]
fn test_r11() {
    let ev = unsafe { crate::RawEvent::new(0x11) };
    let mut workload = cpu_workload;

    test_single(&Event::from(ev), &mut workload);
}

#[test]
#[cfg(target_arch = "aarch64")]
fn test_r1b() {
    let ev = unsafe { crate::RawEvent::new(0x1b) };
    let mut workload = cpu_workload;

    test_single(&Event::from(ev), &mut workload);
}
