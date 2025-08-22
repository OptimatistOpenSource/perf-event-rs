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

use crate::sampling::{
    record::{BasicFlags, SampleId},
    SamplerGroupStat,
};

mod raw;

/// Standalone read record is only generated if it is inherit event with multiple children
/// attr.inherit = 1 && attr.inherit_stat = 1

#[derive(Debug, Clone)]
pub struct ReadRecord {
    pub basic_flags: BasicFlags,
    pub pid: u32,
    pub tid: u32,
    pub values: SamplerGroupStat,
    pub sample_id: Option<SampleId>,
}

impl ReadRecord {
    pub(crate) unsafe fn from_ptr(
        ptr: *const u8,
        sample_type: u64,
        sample_id_all: bool,
        misc: u16,
    ) -> Self {
        let mut raw = raw::Raw {
            read_ptr: ptr,
            sample_type,
        };

        let sized = raw.sized();
        Self {
            basic_flags: BasicFlags::new(misc),
            pid: sized.pid,
            tid: sized.tid,
            values: {
                let (head, values) = raw.values();
                SamplerGroupStat::from_raw(head, values)
            },
            sample_id: sample_id_all.then(|| raw.sample_id()),
        }
    }
}
