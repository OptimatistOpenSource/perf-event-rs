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

use crate::counting::group::tests::test_group;
use crate::test::mem_workload;
use crate::{Event, SoftwareEvent};

#[test]
fn test_page_fault_per_clock() {
    let ev_1 = SoftwareEvent::PageFaults;
    let ev_2 = SoftwareEvent::CpuClock;
    let mut workload = mem_workload;

    test_group(&Event::from(ev_1), &Event::from(ev_2), &mut workload);
}
