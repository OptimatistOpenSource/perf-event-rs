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

use crate::infra::{BoxSliceExt, WrapResult};
use crate::sampling::group::inner::Inner;
use crate::sampling::{ReadFormatHead, ReadFormatValue, SamplerGuard};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read};
use std::{io, slice};

#[derive(Debug, Clone)]
pub struct SamplerGroupStat {
    pub time_enabled: u64,
    pub time_running: u64,
    /// Map of `event_id` -> [`MemberCount`]
    pub member_counts: HashMap<u64, MemberCount>,
}

#[derive(Debug, Clone)]
pub struct MemberCount {
    pub event_count: u64,
    #[cfg(feature = "linux-6.0")]
    pub event_lost: u64,
}

impl SamplerGroupStat {
    pub fn member_count(&self, guard: &SamplerGuard) -> Result<MemberCount, Error> {
        self.member_counts
            .get(&guard.event_id())
            .unwrap()
            .clone()
            .wrap_ok()
    }

    pub(crate) fn from_raw(head: &ReadFormatHead, values: &[ReadFormatValue]) -> Self {
        Self {
            time_enabled: head.time_enabled,
            time_running: head.time_running,
            member_counts: values
                .iter()
                .map(|it| {
                    let member_count = MemberCount {
                        event_count: it.event_count,
                        #[cfg(feature = "linux-6.0")]
                        event_lost: it.event_lost,
                    };
                    (it.event_id, member_count)
                })
                .collect(),
        }
    }
}

#[inline]
pub fn inner_stat(inner: &mut Inner) -> io::Result<SamplerGroupStat> {
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

    SamplerGroupStat::from_raw(head, values).wrap_ok()
}
