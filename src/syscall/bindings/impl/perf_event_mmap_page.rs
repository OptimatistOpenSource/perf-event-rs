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

use crate::debug_struct;
use crate::syscall::bindings::perf_event_mmap_page;
use std::fmt::{Debug, Formatter};

impl Debug for perf_event_mmap_page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_struct! {
            name: perf_event_mmap_page
            self: self
            fmt: f
            fields:
                version
                compat_version
                lock
                index
                offset
                time_enabled
                time_running
                __bindgen_anon_1
                pmc_width
                time_shift
                time_mult
                time_offset
                time_zero
                size
                time_cycles
                time_mask
                data_head
                data_tail
                #[cfg(feature = "linux-4.1")]
                data_offset
                #[cfg(feature = "linux-4.1")]
                data_size
                aux_head
                aux_tail
                aux_offset
                aux_size
        }

        Ok(())
    }
}
