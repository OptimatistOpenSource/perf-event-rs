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

use crate::counting::{Counter, ReadOneValue};
use crate::infra::{SizedExt, WrapResult};
use std::io;
use std::io::Read;
use std::mem::size_of;

#[derive(Debug, Clone)]
pub struct CounterStat {
    pub event_id: u64,
    pub event_count: u64,
    pub time_enabled: u64,
    pub time_running: u64,
}

#[inline]
pub fn counter_stat(counter: &mut Counter) -> io::Result<CounterStat> {
    let mut buf = unsafe { <[u8; size_of::<ReadOneValue>()]>::uninit() };
    counter.file.read_exact(&mut buf)?;

    let layout = unsafe { &*(buf.as_ptr() as *const ReadOneValue) };
    CounterStat {
        event_id: layout.event_id,
        event_count: layout.event_count,
        time_enabled: layout.time_enabled,
        time_running: layout.time_running,
    }
    .wrap_ok()
}
