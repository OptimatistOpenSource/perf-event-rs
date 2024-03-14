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

use crate::perf_event::event::Event;

#[derive(Clone, Debug)]
pub struct TracepointEvent {
    /// The content of `/sys/kernel/debug/tracing/events/*/*/id`
    pub id: u64,
}

impl TracepointEvent {
    pub const fn new(id: u64) -> Self {
        Self { id }
    }
}

impl From<TracepointEvent> for Event {
    fn from(value: TracepointEvent) -> Self {
        Self::Tracepoint(value)
    }
}
