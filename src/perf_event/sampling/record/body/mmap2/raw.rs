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

/*
struct {
  u32    pid;
  u32    tid;
  u64    addr;
  u64    len;
  u64    pgoff;
  union {
      struct {
          u32    maj;
          u32    min;
          u64    ino;
          u64    ino_generation;
      };
      struct {   /* if PERF_RECORD_MISC_MMAP_BUILD_ID */
          u8     build_id_size;
          u8     __reserved_1;
          u16    __reserved_2;
          u8     build_id[20];
      };
  };
  u32    prot;
  u32    flags;
  char   filename[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::sample_id::SampleId;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AnonStruct1 {
    pub maj: u32,
    pub min: u32,
    pub ino: u64,
    pub ino_generation: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AnonStruct2 {
    pub build_id_size: u8,
    __reserved_1: u8,
    __reserved_2: u16,
    pub build_id: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union AnonUnion {
    pub anon_struct_1: AnonStruct1,
    pub anon_struct_2: AnonStruct2,
}

#[repr(C)]
pub struct Sized {
    pub pid: u32,
    pub tid: u32,
    pub addr: u64,
    pub len: u64,
    pub pgoff: u64,
    pub anon_union: AnonUnion,
    pub prot: u32,
    pub flags: u32,
}

pub struct Raw {
    pub read_ptr: *const u8,
    pub sample_type: u64,
}

impl Raw {
    pub unsafe fn sized(&mut self) -> &Sized {
        let ptr = self.read_ptr as *const Sized;
        self.read_ptr = ptr.add(1) as _;
        &*ptr
    }

    pub unsafe fn filename(&mut self) -> &[u8] {
        let ptr = self.read_ptr;
        let zt = ZeroTerminated::from_ptr(ptr);
        let slice = zt.as_slice();
        // Above [u8] will be rounded up to 64-bit in size in the kernel
        self.read_ptr = slice.follow_mem_ptr().align_as_ptr::<u64>() as _;
        slice
    }

    pub unsafe fn sample_id(&self) -> SampleId {
        SampleId::from_ptr(self.read_ptr, self.sample_type)
    }
}
