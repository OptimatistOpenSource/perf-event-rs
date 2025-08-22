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

use crate::sampling::record::{MmapFlags, SampleId};
use std::ffi::CString;

mod raw;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MmapRecord {
    pub flags: MmapFlags,
    pub pid: u32,
    pub tid: u32,
    pub addr: u64,
    pub len: u64,
    pub pgoff: u64,
    pub filename: CString,
    pub sample_id: Option<SampleId>,
}

impl MmapRecord {
    pub(crate) unsafe fn from_ptr(
        ptr: *const u8,
        sample_type: u64,
        sample_id_all: bool,
        misc: u16,
    ) -> Self {
        let raw = &*(ptr as *const raw::Raw);
        let mut sample_id = None;

        if sample_id_all {
            let sample_id_ptr = ptr.add(std::mem::size_of::<raw::Raw>());
            sample_id = Some(SampleId::from_ptr(sample_id_ptr, sample_type));
        }

        Self {
            flags: MmapFlags::new(misc),
            pid: raw.pid,
            tid: raw.tid,
            addr: raw.addr,
            len: raw.len,
            pgoff: raw.pgoff,
            filename: CString::from_vec_unchecked(raw.filename.as_slice().to_vec()),
            sample_id,
        }
    }
}
