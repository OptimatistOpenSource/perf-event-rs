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
pub enum MemOp {
    Na,
    Load,
    Store,
    Pfetch,
    Exec,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MemLvl {
    Na,
    Hit,
    Miss,
    L1,
    Lfb,
    L2,
    L3,
    LocRam,
    RemRam1,
    RemRam2,
    RemCce1,
    RemCce2,
    Io,
    Unc,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MemSnoop {
    Na,
    None,
    Hit,
    Miss,
    Hitm,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MemLock {
    Na,
    Locked,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MemDtlb {
    Na,
    Hit,
    Miss,
    L1,
    L2,
    Wk,
    Os,
}

#[derive(Debug, Clone)]
pub struct DataSrc {
    pub mem_op: MemOp,
    pub mem_lvl: MemLvl,
    pub mem_snoop: MemSnoop,
    pub mem_lock: MemLock,
    pub mem_dtlb: MemDtlb,
}

impl MemOp {
    pub(crate) fn from_raw(raw: u64) -> Self {
        #[rustfmt::skip]
        let val = match (raw >> PERF_MEM_OP_SHIFT) as u32 {
            bits if bits & PERF_MEM_OP_NA     > 0 => Self::Na,
            bits if bits & PERF_MEM_OP_LOAD   > 0 => Self::Load,
            bits if bits & PERF_MEM_OP_STORE  > 0 => Self::Store,
            bits if bits & PERF_MEM_OP_PFETCH > 0 => Self::Pfetch,
            bits if bits & PERF_MEM_OP_EXEC   > 0 => Self::Exec,
            bits => unreachable!("mem_op bits: {}", bits),
        };
        val
    }
}

impl MemLvl {
    pub(crate) fn from_raw(raw: u64) -> Self {
        #[rustfmt::skip]
        let val = match (raw >> PERF_MEM_LVL_SHIFT) as u32 {
            bits if bits & PERF_MEM_LVL_NA       > 0 => Self::Na,
            bits if bits & PERF_MEM_LVL_HIT      > 0 => Self::Hit,
            bits if bits & PERF_MEM_LVL_MISS     > 0 => Self::Miss,
            bits if bits & PERF_MEM_LVL_L1       > 0 => Self::L1,
            bits if bits & PERF_MEM_LVL_LFB      > 0 => Self::Lfb,
            bits if bits & PERF_MEM_LVL_L2       > 0 => Self::L2,
            bits if bits & PERF_MEM_LVL_L3       > 0 => Self::L3,
            bits if bits & PERF_MEM_LVL_LOC_RAM  > 0 => Self::LocRam,
            bits if bits & PERF_MEM_LVL_REM_RAM1 > 0 => Self::RemRam1,
            bits if bits & PERF_MEM_LVL_REM_RAM2 > 0 => Self::RemRam2,
            bits if bits & PERF_MEM_LVL_REM_CCE1 > 0 => Self::RemCce1,
            bits if bits & PERF_MEM_LVL_REM_CCE2 > 0 => Self::RemCce2,
            bits if bits & PERF_MEM_LVL_IO       > 0 => Self::Io,
            bits if bits & PERF_MEM_LVL_UNC      > 0 => Self::Unc,
            bits => unreachable!("mem_lvl bits: {}", bits),
        };
        val
    }
}

impl MemSnoop {
    pub(crate) fn from_raw(raw: u64) -> Self {
        #[rustfmt::skip]
        let val = match (raw >> PERF_MEM_SNOOP_SHIFT) as u32 {
            bits if bits & PERF_MEM_SNOOP_NA   > 0 => Self::Na,
            bits if bits & PERF_MEM_SNOOP_NONE > 0 => Self::None,
            bits if bits & PERF_MEM_SNOOP_HIT  > 0 => Self::Hit,
            bits if bits & PERF_MEM_SNOOP_MISS > 0 => Self::Miss,
            bits if bits & PERF_MEM_SNOOP_HITM > 0 => Self::Hitm,
            bits => unreachable!("mem_snoop bits: {}", bits),
        };
        val
    }
}

impl MemLock {
    pub(crate) fn from_raw(raw: u64) -> Self {
        #[rustfmt::skip]
        let val = match (raw >> PERF_MEM_LOCK_SHIFT) as u32 {
            bits if bits & PERF_MEM_LOCK_NA     > 0 => Self::Na,
            bits if bits & PERF_MEM_LOCK_LOCKED > 0 => Self::Locked,
            bits => unreachable!("mem_lock bits: {}", bits),
        };
        val
    }
}

impl MemDtlb {
    pub(crate) fn from_raw(raw: u64) -> Self {
        #[rustfmt::skip]
        let val = match (raw >> PERF_MEM_TLB_SHIFT) as u32 {
            bits if bits & PERF_MEM_TLB_NA   > 0 => Self::Na,
            bits if bits & PERF_MEM_TLB_HIT  > 0 => Self::Hit,
            bits if bits & PERF_MEM_TLB_MISS > 0 => Self::Miss,
            bits if bits & PERF_MEM_TLB_L1   > 0 => Self::L1,
            bits if bits & PERF_MEM_TLB_L2   > 0 => Self::L2,
            bits if bits & PERF_MEM_TLB_WK   > 0 => Self::Wk,
            bits if bits & PERF_MEM_TLB_OS   > 0 => Self::Os,
            bits => unreachable!("mem_dtlb bits: {}", bits),
        };
        val
    }
}

impl DataSrc {
    pub(crate) fn from_raw(raw: u64) -> Self {
        #[rustfmt::skip]
        let val = Self {
            mem_op:    MemOp   ::from_raw(raw),
            mem_lvl:   MemLvl  ::from_raw(raw),
            mem_snoop: MemSnoop::from_raw(raw),
            mem_lock:  MemLock ::from_raw(raw),
            mem_dtlb:  MemDtlb ::from_raw(raw),
        };
        val
    }
}
