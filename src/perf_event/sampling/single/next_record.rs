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

use crate::infra::{SizedExt, WrapOption};
use crate::sampling::record::*;
use crate::sampling::Sampler;
use crate::syscall::bindings::*;
use std::alloc::{alloc, dealloc, Layout};
use std::slice;

#[inline]
pub fn next_record(sampler: &mut Sampler) -> Option<Record> {
    let metapage =
        unsafe { (sampler.mmap.as_mut_ptr() as *mut perf_event_mmap_page).as_mut() }.unwrap();
    let data_size = sampler.data_size;

    // Read data_head with volatile access and acquire semantics
    let data_head = unsafe { std::ptr::read_volatile(&metapage.data_head) };
    // Issue read memory barrier (rmb) after reading data_head
    std::sync::atomic::fence(std::sync::atomic::Ordering::Acquire);

    let data_tail = unsafe { std::ptr::read_volatile(&metapage.data_tail) };

    // data_head continuously increases and doesn't wrap, so we need to manually wrap it
    let data_head_wrapped = data_head % data_size;

    if data_tail == data_head_wrapped {
        return None;
    }

    let data_ptr = unsafe { sampler.mmap.as_mut_ptr().add(sampler.data_offset as _) };

    let record_len = match data_tail as isize + 8 - data_size as isize {
        left if left <= 0 => {
            let offset = (data_tail + 6) as _;
            let ptr = unsafe { data_ptr.add(offset) } as *const u16;
            unsafe { *ptr }
        }
        1 => unsafe {
            let mut buf = <[u8; 2]>::uninit();
            buf[0] = *(data_ptr.add((data_size - 1) as _) as *const u8);
            buf[1] = *(data_ptr as *const u8);
            std::mem::transmute::<[u8; 2], u16>(buf)
        },
        left => unsafe {
            let ptr = data_ptr.add((left - 2) as _) as *const u16;
            *ptr
        },
    } as usize;

    let mut dealloc_record_buf = false;
    let record_buf = match data_tail as isize + record_len as isize - data_size as isize {
        left if left > 0 => unsafe {
            let buf = {
                dealloc_record_buf = true;
                let layout = Layout::array::<u8>(record_len).unwrap();
                alloc(layout)
            };

            let ring_end_part_ptr = data_ptr.add(data_tail as _);
            let ring_end_part_len = (data_size - data_tail) as usize;
            std::ptr::copy_nonoverlapping(ring_end_part_ptr, buf, ring_end_part_len);

            let ring_start_part_ptr = data_ptr;
            let ring_start_part_len = left as _;
            std::ptr::copy_nonoverlapping(
                ring_start_part_ptr,
                buf.add(ring_end_part_len),
                ring_start_part_len,
            );

            slice::from_raw_parts(buf, record_len)
        },
        _ => unsafe { slice::from_raw_parts(data_ptr.add(data_tail as _), record_len) },
    };

    let record_header =
        unsafe { (record_buf.as_ptr() as *const perf_event_header).as_ref() }.unwrap();
    let record = unsafe {
        let follow_mem_ptr = (record_header as *const perf_event_header).add(1) as *const _;
        match record_header.type_ {
            PERF_RECORD_MMAP => Record::Mmap(MmapRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            PERF_RECORD_LOST => Record::Lost(LostRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            PERF_RECORD_COMM => Record::Comm(CommRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            PERF_RECORD_EXIT => Record::Exit(ExitRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            PERF_RECORD_THROTTLE => Record::Throttle(ThrottleRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            PERF_RECORD_UNTHROTTLE => Record::Unthrottle(UnthrottleRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            PERF_RECORD_FORK => Record::Fork(ForkRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            PERF_RECORD_READ => Record::Read(ReadRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            PERF_RECORD_SAMPLE => Record::Sample(SampleRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.regs_user_len,
                #[cfg(feature = "linux-3.19")]
                sampler.regs_intr_len,
                record_header.misc,
            )),
            #[cfg(feature = "linux-3.12")]
            PERF_RECORD_MMAP2 => Record::Mmap2(Mmap2Record::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-4.1")]
            PERF_RECORD_AUX => Record::Aux(AuxRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-4.1")]
            PERF_RECORD_ITRACE_START => Record::ItraceStart(ItraceStartRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-4.2")]
            PERF_RECORD_LOST_SAMPLES => Record::LostSamples(LostSamplesRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-4.3")]
            PERF_RECORD_SWITCH => Record::Switch(SwitchRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-4.3")]
            PERF_RECORD_SWITCH_CPU_WIDE => Record::SwitchCpuWide(SwitchCpuWideRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-4.12")]
            PERF_RECORD_NAMESPACES => Record::Namespaces(NamespacesRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-5.1")]
            PERF_RECORD_KSYMBOL => Record::Ksymbol(KsymbolRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-5.1")]
            PERF_RECORD_BPF_EVENT => Record::BpfEvent(BpfEventRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-5.7")]
            PERF_RECORD_CGROUP => Record::Cgroup(CgroupRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-5.9")]
            PERF_RECORD_TEXT_POKE => Record::TextPoke(TextPokeRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            #[cfg(feature = "linux-5.16")]
            PERF_RECORD_AUX_OUTPUT_HW_ID => Record::AuxOutputHwId(AuxOutputHwIdRecord::from_ptr(
                follow_mem_ptr,
                sampler.sample_type,
                sampler.sample_id_all,
                record_header.misc,
            )),
            _ => unreachable!(),
        }
    };

    if dealloc_record_buf {
        let layout = Layout::array::<u8>(record_len).unwrap();
        #[allow(clippy::as_ptr_cast_mut)]
        unsafe {
            dealloc(record_buf.as_ptr() as _, layout)
        }
    }

    // Update data_tail with store release semantics to ensure the kernel sees the update
    let new_data_tail = (data_tail + record_len as u64) % data_size;
    // Issue write memory barrier (wmb) before writing data_tail
    std::sync::atomic::fence(std::sync::atomic::Ordering::Release);
    unsafe { std::ptr::write_volatile(&mut metapage.data_tail, new_data_tail) };

    record.wrap_some()
}
