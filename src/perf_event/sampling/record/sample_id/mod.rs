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

mod raw;

#[derive(Debug, Clone)]
pub struct SampleId {
    pub pid: Option<u32>,
    pub tid: Option<u32>,
    pub time: Option<u64>,
    pub id_1: Option<u64>,
    pub stream_id: Option<u64>,
    pub cpu: Option<u32>,
    #[cfg(feature = "linux-3.12")]
    pub id_2: Option<u64>,
}

impl SampleId {
    pub(crate) unsafe fn from_ptr(ptr: *const u8, sample_type: u64) -> Self {
        let mut raw = raw::Raw {
            read_ptr: ptr,
            sample_type,
        };

        Self {
            pid: raw.pid().cloned(),
            tid: raw.tid().cloned(),
            time: raw.time().cloned(),
            id_1: raw.id_1().cloned(),
            stream_id: raw.stream_id().cloned(),
            cpu: raw.cpu().cloned(),
            #[cfg(feature = "linux-3.12")]
            id_2: raw.id_2().cloned(),
        }
    }
}
