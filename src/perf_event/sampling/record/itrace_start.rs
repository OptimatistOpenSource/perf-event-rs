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

/*
struct {
  u32    pid;
  u32    tid;
};
*/

use crate::sampling::record::{BasicFlags, SampleId};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ItraceStartRecord {
    pub basic_flags: BasicFlags,
    pub pid: u32,
    pub tid: u32,
    pub sample_id: Option<SampleId>,
}

impl ItraceStartRecord {
    pub(crate) unsafe fn from_ptr(
        ptr: *const u8,
        sample_type: u64,
        sample_id_all: bool,
        misc: u16,
    ) -> Self {
        let body = (ptr as *const Body).read();
        let mut sample_id = None;

        if sample_id_all {
            let sample_id_ptr = ptr.add(std::mem::size_of::<Body>());
            sample_id = Some(SampleId::from_ptr(sample_id_ptr, sample_type));
        }

        Self {
            basic_flags: BasicFlags::new(misc),
            pid: body.pid,
            tid: body.tid,
            sample_id,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
struct Body {
    pub pid: u32,
    pub tid: u32,
}
