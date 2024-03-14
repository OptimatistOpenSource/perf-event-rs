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

#[allow(clippy::useless_transmute)]
#[allow(clippy::unnecessary_cast)]
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]
#[rustfmt::skip]
mod bindgen;
mod r#impl;

#[rustfmt::skip]
pub use bindgen::*;
#[allow(unused_imports)]
pub use r#impl::*;

#[cfg(feature = "linux-5.12")]
#[allow(non_camel_case_types)]
pub type perf_sample_weight = bindgen::perf_sample_weight;

#[cfg(not(feature = "linux-5.12"))]
#[allow(non_camel_case_types)]
pub type perf_sample_weight = u64;
