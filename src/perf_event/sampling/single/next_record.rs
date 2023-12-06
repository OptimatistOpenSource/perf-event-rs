use crate::infra::{ArrayExt, VecExt, WrapBox, WrapOption};
use crate::sampling::record::*;
use crate::sampling::Sampling;
use crate::syscall::bindings::*;
use std::slice;

pub fn next_record(sampling: &mut Sampling) -> Option<Record> {
    let metapage =
        unsafe { (sampling.mmap.as_mut_ptr() as *mut perf_event_mmap_page).as_mut() }.unwrap();
    let data_size = metapage.data_size;
    let data_head = metapage.data_head % data_size;
    let data_tail = metapage.data_tail;

    let ring_ptr = unsafe { sampling.mmap.as_mut_ptr().add(metapage.data_offset as _) };

    if data_tail == data_head {
        return None;
    }

    let record_len = match data_tail as isize + 8 - data_size as isize {
        left if left <= 0 => {
            let offset = (data_tail + 6) as _;
            let ptr = unsafe { ring_ptr.add(offset) } as *const u16;
            unsafe { *ptr }
        }
        1 => unsafe {
            let mut buf = <[u8; 2]>::uninit();
            buf[0] = *(ring_ptr.add((data_size - 1) as _) as *const u8);
            buf[1] = *(ring_ptr as *const u8);
            std::mem::transmute(buf)
        },
        left => unsafe {
            let ptr = ring_ptr.add((left - 2) as _) as *const u16;
            *ptr
        },
    } as usize;

    let record_buf = match data_tail as isize + record_len as isize - data_size as isize {
        left if left > 0 => {
            let ring_end_part = {
                let start = data_tail as _;
                let len = (data_size - data_tail) as usize;
                unsafe { slice::from_raw_parts(ring_ptr.add(start), len) }
            };
            let ring_start_part = unsafe { slice::from_raw_parts(ring_ptr, left as _) };

            let mut buf = unsafe { Vec::with_len_uninit(record_len) };
            ring_end_part
                .iter()
                .chain(ring_start_part)
                .enumerate()
                .for_each(|(i, byte)| buf[i] = *byte);
            buf
        }
        _ => unsafe { slice::from_raw_parts(ring_ptr.add(data_tail as _), record_len) }.to_vec(),
    };

    metapage.data_tail = (data_tail + record_len as u64) % data_size;

    let record_header =
        unsafe { (record_buf.as_ptr() as *const perf_event_header).as_ref() }.unwrap();
    #[allow(non_upper_case_globals)]
    let record_body = unsafe {
        let follow_mem_ptr = (record_header as *const perf_event_header).add(1) as *const _;
        match record_header.type_ {
            perf_event_type_PERF_RECORD_MMAP => {
                RecordBody::Mmap(mmap::Body::from_ptr(follow_mem_ptr).wrap_box())
            }
            perf_event_type_PERF_RECORD_LOST => {
                let ptr = follow_mem_ptr as *const lost::Body;
                RecordBody::Lost(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_COMM => {
                RecordBody::Comm(comm::Body::from_ptr(follow_mem_ptr).wrap_box())
            }
            perf_event_type_PERF_RECORD_EXIT => {
                let ptr = follow_mem_ptr as *const exit::Body;
                RecordBody::Exit(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_THROTTLE => {
                let ptr = follow_mem_ptr as *const throttle::Body;
                RecordBody::Throttle(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_UNTHROTTLE => {
                let ptr = follow_mem_ptr as *const unthrottle::Body;
                RecordBody::Unthrottle(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_FORK => {
                let ptr = follow_mem_ptr as *const fork::Body;
                RecordBody::Fork(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_READ => {
                RecordBody::Read(read::Body::from_ptr(follow_mem_ptr).wrap_box())
            }
            perf_event_type_PERF_RECORD_SAMPLE => RecordBody::Sample(
                sample::Body::from_ptr(
                    follow_mem_ptr,
                    sampling.user_regs_len,
                    sampling.intr_regs_len,
                )
                .wrap_box(),
            ),
            perf_event_type_PERF_RECORD_MMAP2 => RecordBody::Mmap2(
                mmap2::Body::from_ptr(follow_mem_ptr, record_header.misc).wrap_box(),
            ),
            perf_event_type_PERF_RECORD_AUX => {
                let ptr = follow_mem_ptr as *const aux::Body;
                RecordBody::Aux(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_ITRACE_START => {
                let ptr = follow_mem_ptr as *const intrace_start::Body;
                RecordBody::ItraceStart(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_LOST_SAMPLES => {
                let ptr = follow_mem_ptr as *const lost_samples::Body;
                RecordBody::LostSamples(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_SWITCH => {
                let ptr = follow_mem_ptr as *const switch::Body;
                RecordBody::Switch(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_SWITCH_CPU_WIDE => {
                let ptr = follow_mem_ptr as *const switch_cpu_wide::Body;
                RecordBody::SwitchCpuWide(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_NAMESPACES => {
                RecordBody::Namespaces(namespaces::Body::from_ptr(follow_mem_ptr).wrap_box())
            }
            perf_event_type_PERF_RECORD_KSYMBOL => {
                RecordBody::Ksymbol(ksymbol::Body::from_ptr(follow_mem_ptr).wrap_box())
            }
            perf_event_type_PERF_RECORD_BPF_EVENT => {
                let ptr = follow_mem_ptr as *const bpf_event::Body;
                RecordBody::BpfEvent(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_CGROUP => {
                RecordBody::Cgroup(cgroup::Body::from_ptr(follow_mem_ptr).wrap_box())
            }
            perf_event_type_PERF_RECORD_TEXT_POKE => {
                RecordBody::TextPoke(text_poke::Body::from_ptr(follow_mem_ptr).wrap_box())
            }
            perf_event_type_PERF_RECORD_AUX_OUTPUT_HW_ID => {
                let ptr = follow_mem_ptr as *const aux_output_hw_id::Body;
                RecordBody::AuxOutputHwId(ptr.read().wrap_box())
            }
            _ => unreachable!(),
        }
    };

    Record {
        misc: record_header.misc,
        body: record_body,
    }
    .wrap_some()
}