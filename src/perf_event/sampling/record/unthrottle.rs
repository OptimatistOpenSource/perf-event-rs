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

use crate::sampling::record::{BasicFlags, SampleId};

// Unthrottle is same to Throttle
#[derive(Debug, Clone)]
pub struct UnthrottleRecord {
    pub basic_flags: BasicFlags,
    pub time: u64,
    pub id: u64,
    pub stream_id: u64,
    pub sample_id: Option<SampleId>,
}

impl UnthrottleRecord {
    pub(crate) unsafe fn from_ptr(
        ptr: *const u8,
        sample_type: u64,
        sample_id_all: bool,
        misc: u16,
    ) -> Self {
        // Unthrottle has the same structure as Throttle
        let raw = &*(ptr as *const crate::sampling::record::throttle::raw::Raw);

        Self {
            basic_flags: BasicFlags::new(misc),
            time: raw.time,
            id: raw.id,
            stream_id: raw.stream_id,
            sample_id: sample_id_all.then(|| raw.sample_id(sample_type)),
        }
    }
}
