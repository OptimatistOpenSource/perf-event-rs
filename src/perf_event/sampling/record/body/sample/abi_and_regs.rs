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

use crate::syscall::bindings::*;

#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Abi {
    AbiNone,
    Abi32,
    Abi64,
}

impl Abi {
    pub(crate) fn from_raw(abi: u64) -> Self {
        #[rustfmt::skip]
        let val = match abi as _ {
            PERF_SAMPLE_REGS_ABI_NONE => Self::AbiNone,
            PERF_SAMPLE_REGS_ABI_32   => Self::Abi32,
            PERF_SAMPLE_REGS_ABI_64   => Self::Abi64,
            abi => unreachable!("ABI: {}", abi),
        };
        val
    }
}

#[derive(Debug, Clone)]
pub struct AbiAndRegs {
    pub abi: Abi,
    pub regs: Vec<u64>,
}

impl AbiAndRegs {
    pub(crate) fn from_raw(raw: (&u64, &[u64])) -> Self {
        let (abi, regs) = raw;
        Self {
            abi: Abi::from_raw(*abi),
            regs: regs.to_vec(),
        }
    }
}
