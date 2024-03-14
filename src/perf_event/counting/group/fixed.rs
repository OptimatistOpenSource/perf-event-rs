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

use crate::counting::group::inner::Inner;
use crate::counting::CounterGroupStat;
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct FixedCounterGroup {
    inner: Arc<RwLock<Inner>>,
}

impl FixedCounterGroup {
    pub(crate) const fn new(inner: Arc<RwLock<Inner>>) -> Self {
        Self { inner }
    }

    fn inner(&self) -> RwLockReadGuard<'_, Inner> {
        self.inner.read().unwrap()
    }

    fn inner_mut(&self) -> RwLockWriteGuard<'_, Inner> {
        self.inner.write().unwrap()
    }

    pub fn enable(&self) -> io::Result<()> {
        self.inner().enable()
    }

    pub fn disable(&self) -> io::Result<()> {
        self.inner().disable()
    }

    pub fn reset(&self) -> io::Result<()> {
        self.inner().reset()
    }

    pub fn stat(&mut self) -> io::Result<CounterGroupStat> {
        self.inner_mut().stat()
    }
}
