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

use crate::Event;

#[derive(Debug, Clone)]
pub struct RawEvent {
    config: u64,
}

impl RawEvent {
    /// # Safety
    /// The `config` argument must be valid for counting mode.
    pub const unsafe fn new(config: u64) -> Self {
        Self { config }
    }

    pub const fn as_u64(&self) -> u64 {
        self.config
    }
}

impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        Self::Raw(value)
    }
}
