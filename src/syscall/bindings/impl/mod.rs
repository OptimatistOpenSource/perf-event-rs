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

mod perf_event_attr;
#[allow(non_snake_case)]
mod perf_event_attr__bindgen_ty_1;
#[allow(non_snake_case)]
mod perf_event_attr__bindgen_ty_2;
#[allow(non_snake_case)]
mod perf_event_attr__bindgen_ty_3;
#[allow(non_snake_case)]
mod perf_event_attr__bindgen_ty_4;
#[cfg(feature = "linux-5.9")]
mod perf_event_mmap_page;
#[cfg(feature = "linux-5.9")]
#[allow(non_snake_case)]
mod perf_event_mmap_page__bindgen_ty_1;
#[cfg(feature = "linux-5.12")]
mod perf_sample_weight;

#[allow(unused_imports)]
pub use perf_event_attr::*;
#[allow(unused_imports)]
pub use perf_event_attr__bindgen_ty_1::*;
#[allow(unused_imports)]
pub use perf_event_attr__bindgen_ty_2::*;
#[allow(unused_imports)]
pub use perf_event_attr__bindgen_ty_3::*;
#[allow(unused_imports)]
pub use perf_event_attr__bindgen_ty_4::*;
#[cfg(feature = "linux-5.9")]
#[allow(unused_imports)]
pub use perf_event_mmap_page::*;
#[cfg(feature = "linux-5.9")]
#[allow(unused_imports)]
pub use perf_event_mmap_page__bindgen_ty_1::*;

#[macro_export]
macro_rules! debug_union {
    (
        name: $name: ident
        self: $self: ident
        fmt: $f: ident
        fields: $($(#[$attr: meta])* $field: ident)+
    ) => {{
        let mut ds = $f.debug_struct(stringify!($name));
        $(
            $(#[$attr])*
            ds.field(stringify!($field), &unsafe { $self.$field });
        )+
        ds.finish()?;
    }};
}

#[macro_export]
macro_rules! debug_struct {
    (
        name: $name: ident
        self: $self: ident
        fmt: $f: ident
        fields: $($(#[$attr: meta])* $field:ident)+
    ) => {{
        let mut ds = $f.debug_struct(stringify!($name));
        $(
            $(#[$attr])*
            ds.field(stringify!($field), &$self.$field);
        )+
        ds.finish()?;
    }};
}

#[macro_export]
macro_rules! debug_struct_fn {
    (
        name: $name: ident
        self: $self: ident
        fmt: $f: ident
        fields: $($(#[$attr: meta])* $field:ident)+
    ) => {{
        let mut ds = $f.debug_struct(stringify!($name));
        $(
            $(#[$attr])*
            ds.field(stringify!($field), &$self.$field());
        )+
        ds.finish()?;
    }};
}
