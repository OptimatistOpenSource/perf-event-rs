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

use crate::syscall::bindings::perf_sample_weight;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum WeightRepr {
    Full,
    #[cfg(feature = "linux-5.12")]
    Vars,
}

#[derive(Debug, Clone)]
pub enum Weight {
    Full(u64),
    #[cfg(feature = "linux-5.12")]
    Vars {
        var1_dw: u32,
        var2_w: u16,
        var3_w: u16,
    },
}

impl Weight {
    pub(crate) const fn from_raw(raw: perf_sample_weight, repr: WeightRepr) -> Self {
        match repr {
            #[cfg(feature = "linux-5.12")]
            WeightRepr::Full => unsafe { Self::Full(raw.full) },
            #[cfg(feature = "linux-5.12")]
            WeightRepr::Vars => unsafe {
                Self::Vars {
                    var1_dw: raw.__bindgen_anon_1.var1_dw,
                    var2_w: raw.__bindgen_anon_1.var2_w,
                    var3_w: raw.__bindgen_anon_1.var3_w,
                }
            },
            #[cfg(not(feature = "linux-5.12"))]
            WeightRepr::Full => Self::Full(raw),
        }
    }
}
