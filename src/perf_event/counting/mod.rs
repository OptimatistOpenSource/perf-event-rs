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

mod config;
mod group;
mod single;

#[allow(unused_imports)]
pub use config::*;
pub use group::*;
pub use single::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct ReadFormatHead {
    pub members_len: u64,  // u64 nr;
    pub time_enabled: u64, // u64 time_enabled;
    pub time_running: u64, // u64 time_running;
                           // ReadFormatValue values[nr];
}

#[repr(C)]
#[derive(Debug, Clone)]
pub(crate) struct ReadFormatValue {
    pub event_count: u64, // u64 value;
    pub event_id: u64,    // u64 id;
}
