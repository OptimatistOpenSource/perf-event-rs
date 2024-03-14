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
struct sample_id {
  { u32 pid, tid; }   /* if PERF_SAMPLE_TID set */
  { u64 time;     }   /* if PERF_SAMPLE_TIME set */
  { u64 id;       }   /* if PERF_SAMPLE_ID set */
  { u64 stream_id;}   /* if PERF_SAMPLE_STREAM_ID set  */
  { u32 cpu, res; }   /* if PERF_SAMPLE_CPU set */
  { u64 id;       }   /* if PERF_SAMPLE_IDENTIFIER set */
};
*/

use crate::syscall::bindings::*;
use std::mem::size_of;
use std::ops::Not;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Raw {
    pub read_ptr: *const u8,
    pub sample_type: u64,
}

type Mask = perf_event_sample_format;

macro_rules! gen_fn {
    ($ty:ty, $name:ident $mask:expr) => {
        #[inline]
        pub unsafe fn $name(&mut self) -> Option<&$ty> {
            self.get_if($mask)
        }
    };
}

impl Raw {
    #[inline]
    #[allow(clippy::unnecessary_cast)] // mask may be u64 or u32 in different linux headers
    const fn is_enabled(&self, mask: Mask) -> bool {
        (self.sample_type & mask as u64) > 0
    }

    #[inline]
    unsafe fn get_if<T>(&mut self, mask: Mask) -> Option<&T> {
        if self.is_enabled(mask).not() {
            return None;
        }
        let ptr = self.read_ptr as *const T;
        self.read_ptr = self.read_ptr.add(size_of::<T>());
        ptr.as_ref()
    }

    gen_fn! { u32, pid       PERF_SAMPLE_TID       }
    gen_fn! { u32, tid       PERF_SAMPLE_TID       }
    gen_fn! { u64, time      PERF_SAMPLE_TIME      }
    gen_fn! { u64, id_1      PERF_SAMPLE_ID        }
    gen_fn! { u64, stream_id PERF_SAMPLE_STREAM_ID }

    pub unsafe fn cpu(&mut self) -> Option<&u32> {
        if self.is_enabled(PERF_SAMPLE_CPU).not() {
            return None;
        }

        let cpu_ptr = self.read_ptr as *const u32;
        self.read_ptr = cpu_ptr.add(2) as _; // skip 32-bit res
        cpu_ptr.as_ref()
    }

    #[cfg(feature = "linux-3.12")]
    gen_fn! { u64, id_2 PERF_SAMPLE_IDENTIFIER }
}
