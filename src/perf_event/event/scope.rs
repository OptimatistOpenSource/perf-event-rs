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

use crate::perf_event::RawAttr;
use std::ops::Not;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum EventScope {
    User,
    Kernel,
    Hv,
    Idle,
    Host,
    Guest,
}

impl EventScope {
    pub fn all() -> Vec<Self> {
        vec![
            Self::User,
            Self::Kernel,
            Self::Hv,
            Self::Idle,
            Self::Host,
            Self::Guest,
        ]
    }

    pub fn all_but_exclude<'t>(scopes: impl IntoIterator<Item = &'t Self>) -> Vec<Self> {
        let excludes = scopes.into_iter().collect::<Vec<_>>();
        Self::all()
            .iter()
            .filter(|s| excludes.contains(s).not())
            .cloned()
            .collect()
    }

    pub(crate) fn enable_in_raw_attr(&self, raw_attr: &mut RawAttr) {
        #[rustfmt::skip]
        match self {
            Self::User   => raw_attr.set_exclude_user(0),
            Self::Kernel => raw_attr.set_exclude_kernel(0),
            Self::Hv     => raw_attr.set_exclude_hv(0),
            Self::Idle   => raw_attr.set_exclude_idle(0),
            Self::Host   => raw_attr.set_exclude_host(0),
            Self::Guest  => raw_attr.set_exclude_guest(0),
        };
    }
}
