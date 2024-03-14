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
  u32    pid, tid;
  struct read_format values;
  struct sample_id sample_id;
};
*/

use crate::infra::SliceExt;
use crate::sampling::record::sample_id::SampleId;
use crate::sampling::{ReadFormatHead, ReadFormatValue};
use std::slice;

#[repr(C)]
pub struct Sized {
    pub pid: u32,
    pub tid: u32,
}

pub struct Raw {
    pub read_ptr: *const u8,
    pub sample_type: u64,
}

impl Raw {
    pub unsafe fn sized(&mut self) -> &Sized {
        let ptr = self.read_ptr as *const Sized;
        self.read_ptr = ptr.add(1) as _;
        &*ptr
    }

    pub unsafe fn values(&mut self) -> (&ReadFormatHead, &[ReadFormatValue]) {
        let head_ptr = self.read_ptr as *const ReadFormatHead;
        let head = &*head_ptr;
        let body_ptr = head_ptr.add(1) as *const ReadFormatValue;
        let slice = slice::from_raw_parts(body_ptr, head.members_len as _);
        self.read_ptr = slice.follow_mem_ptr() as _;
        (head, slice)
    }

    pub unsafe fn sample_id(&self) -> SampleId {
        SampleId::from_ptr(self.read_ptr, self.sample_type)
    }
}
