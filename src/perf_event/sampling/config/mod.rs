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

mod extra_config;
mod extra_record;
mod new;
mod sample_record_fields;

use crate::perf_event::RawAttr;
use crate::{Event, EventScope};
use std::ffi::CString;
use std::fmt::Debug;
use std::rc::Rc;

pub use extra_config::*;
pub use extra_record::*;
pub use sample_record_fields::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum OverflowBy {
    Period(u64),
    Freq(u64),
}

#[derive(Debug, Clone)]
pub struct Config {
    // This will keep the ptr of `kprobe_func` or `uprobe_path` valid if present.
    #[allow(dead_code)]
    kprobe_func_or_uprobe_path: Option<Rc<CString>>,
    raw_attr: RawAttr,
}

impl Config {
    pub fn new<'t>(
        event: &Event,
        scopes: impl IntoIterator<Item = &'t EventScope>,
        overflow_by: &OverflowBy,
    ) -> Self {
        Self::extra_new(event, scopes, overflow_by, &Default::default())
    }

    pub fn extra_new<'t>(
        event: &Event,
        scopes: impl IntoIterator<Item = &'t EventScope>,
        overflow_by: &OverflowBy,
        extra_config: &ExtraConfig,
    ) -> Self {
        new::new(event, scopes, overflow_by, extra_config)
    }

    /// Construct from a raw `perf_event_attr` struct.
    /// # Safety
    /// The `raw_attr` argument must be a properly initialized
    /// `perf_event_attr` struct for counting mode.
    pub const unsafe fn from_raw(raw_attr: RawAttr) -> Self {
        Self {
            kprobe_func_or_uprobe_path: None,
            raw_attr,
        }
    }

    pub fn into_raw(self) -> RawAttr {
        self.raw_attr
    }

    pub const fn as_raw(&self) -> &RawAttr {
        &self.raw_attr
    }
}
