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

use crate::counting::group::guard::CounterGuard;
use crate::counting::group::inner::Inner;
use crate::counting::{ReadFormatHead, ReadFormatValue};
use crate::infra::{BoxSliceExt, WrapResult};
use std::collections::HashMap;
use std::io::{ErrorKind, Read};
use std::{io, slice};

#[derive(Debug, Clone)]
pub struct CounterGroupStat {
    pub time_enabled: u64,
    pub time_running: u64,
    /// Map of `event_id` -> `event_count`
    pub member_counts: HashMap<u64, u64>,
}

impl CounterGroupStat {
    pub fn member_count(&self, guard: &CounterGuard) -> io::Result<u64> {
        (*self.member_counts.get(&guard.event_id()).unwrap()).wrap_ok()
    }

    pub(crate) fn from_raw(head: &ReadFormatHead, values: &[ReadFormatValue]) -> Self {
        Self {
            time_enabled: head.time_enabled,
            time_running: head.time_running,
            member_counts: values
                .iter()
                .map(|it| (it.event_id, it.event_count))
                .collect(),
        }
    }
}

#[inline]
pub fn inner_stat(inner: &mut Inner) -> io::Result<CounterGroupStat> {
    let members_len = inner.members.len();
    let Some(leader) = inner.leader_mut() else {
        return Err(io::Error::new(ErrorKind::Other, "Group has no members"));
    };

    use std::mem::size_of;

    let buf = {
        let len = size_of::<ReadFormatHead>() + size_of::<ReadFormatValue>() * members_len;

        let mut buf = unsafe { Box::<[u8]>::uninit(len) };
        leader.file.read_exact(&mut buf)?;

        buf
    };

    let head = unsafe { &*(buf.as_ptr() as *const ReadFormatHead) };
    let values = {
        let head_ptr = head as *const ReadFormatHead;
        let values_ptr = unsafe { head_ptr.add(1) as *const ReadFormatValue };
        unsafe { slice::from_raw_parts(values_ptr, inner.members.len()) }
    };

    CounterGroupStat::from_raw(head, values).wrap_ok()
}
