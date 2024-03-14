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

mod new;

use crate::perf_event::PerfEventAttr;
use crate::{Event, EventScope};
use std::ffi::CString;
use std::rc::Rc;

pub type ExtraConfig = crate::sampling::ExtraConfig;

#[derive(Debug, Clone)]
pub struct Config {
    // This will keep the ptr of `kprobe_func` or `uprobe_path` valid if present.
    #[allow(dead_code)]
    kprobe_func_or_uprobe_path: Option<Rc<CString>>,
    perf_event_attr: PerfEventAttr,
}

impl Config {
    pub fn new<'t>(event: &Event, scopes: impl IntoIterator<Item = &'t EventScope>) -> Self {
        Self::extra_new(event, scopes, &Default::default())
    }

    pub fn extra_new<'t>(
        event: &Event,
        scopes: impl IntoIterator<Item = &'t EventScope>,
        extra_config: &ExtraConfig,
    ) -> Self {
        new::new(event, scopes, extra_config)
    }

    /// Construct from a `PerfEventAttr` struct.
    /// # Safety
    /// The `perf_event_attr` argument must be properly initialized from
    /// `PerfEventAttr` struct for counting mode.
    pub const unsafe fn from_raw(perf_event_attr: PerfEventAttr) -> Self {
        Self {
            kprobe_func_or_uprobe_path: None,
            perf_event_attr,
        }
    }

    pub fn into_raw(self) -> PerfEventAttr {
        self.perf_event_attr
    }

    pub const fn as_raw(&self) -> &PerfEventAttr {
        &self.perf_event_attr
    }
}
