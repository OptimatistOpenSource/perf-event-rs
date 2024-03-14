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
  u64    aux_offset;
  u64    aux_size;
  u64    flags;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Raw {
    pub aux_offset: u64,
    pub aux_size: u64,
    pub flags: u64,
}

impl Raw {
    pub unsafe fn sample_id(&self, sample_type: u64) -> SampleId {
        let ptr = (self as *const Self).add(1) as _;
        SampleId::from_ptr(ptr, sample_type)
    }
}
