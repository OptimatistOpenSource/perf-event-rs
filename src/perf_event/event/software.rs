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

use crate::syscall::bindings::*;
use crate::Event;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SoftwareEvent {
    CpuClock,
    TaskClock,
    PageFaults,
    ContextSwitches,
    CpuMigrations,
    PageFaultsMin,
    PageFaultsMaj,
    AlignmentFaults,
    EmulationFaults,
    #[cfg(feature = "linux-3.12")]
    Dummy,
    #[cfg(feature = "linux-4.4")]
    BpfOutput,
    #[cfg(feature = "linux-5.13")]
    CgroupSwitches,
}

impl SoftwareEvent {
    pub(crate) const fn as_u64(&self) -> u64 {
        use SoftwareEvent::*;
        #[rustfmt::skip]
        let config = match self {
            CpuClock        => PERF_COUNT_SW_CPU_CLOCK,
            TaskClock       => PERF_COUNT_SW_TASK_CLOCK,
            PageFaults      => PERF_COUNT_SW_PAGE_FAULTS,
            ContextSwitches => PERF_COUNT_SW_CONTEXT_SWITCHES,
            CpuMigrations   => PERF_COUNT_SW_CPU_MIGRATIONS,
            PageFaultsMin   => PERF_COUNT_SW_PAGE_FAULTS_MIN,
            PageFaultsMaj   => PERF_COUNT_SW_PAGE_FAULTS_MAJ,
            AlignmentFaults => PERF_COUNT_SW_ALIGNMENT_FAULTS,
            EmulationFaults => PERF_COUNT_SW_EMULATION_FAULTS,
            #[cfg(feature = "linux-3.12")]
            Dummy           => PERF_COUNT_SW_DUMMY,
            #[cfg(feature = "linux-4.4")]
            BpfOutput       => PERF_COUNT_SW_BPF_OUTPUT,
            #[cfg(feature = "linux-5.13")]
            CgroupSwitches  => PERF_COUNT_SW_CGROUP_SWITCHES,
        };
        config as _
    }
}

impl From<SoftwareEvent> for Event {
    fn from(value: SoftwareEvent) -> Self {
        Self::Software(value)
    }
}
