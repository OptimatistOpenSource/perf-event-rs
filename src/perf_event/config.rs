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

use std::{io, result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("PID is invalid: {0}")]
    InvalidPid(u32),
    #[error("Measures any process on any cpu is invalid")]
    InvalidProcessCpu,
    #[error("Failed to perform perf_event_open: {0}")]
    SyscallFailed(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

pub enum Process {
    Any,     // -1
    Current, // 0
    Pid(u32),
}

impl Process {
    pub(crate) const fn as_i32(&self) -> Result<i32> {
        match self {
            Self::Any => Ok(-1),
            Self::Current => Ok(0),
            Self::Pid(0) => Err(Error::InvalidPid(0)),
            Self::Pid(n) => Ok(*n as _),
        }
    }
}

pub enum Cpu {
    Any, // -1
    Id(u32),
}

impl Cpu {
    pub(crate) const fn as_i32(&self) -> i32 {
        match self {
            Self::Any => -1,
            Self::Id(n) => *n as _,
        }
    }
}
