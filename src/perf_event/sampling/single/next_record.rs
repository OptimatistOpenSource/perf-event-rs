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
                let record = mmap::Body::from_ptr(follow_mem_ptr);
                RecordBody::Mmap(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_LOST => {
                let record = lost::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Lost(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_COMM => {
                let record = comm::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Comm(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_EXIT => {
                let record = exit::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Exit(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_THROTTLE => {
                let record = throttle::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Throttle(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_UNTHROTTLE => {
                let record = unthrottle::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Unthrottle(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_FORK => {
                let record = fork::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Fork(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_READ => {
                let record = read::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Read(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_SAMPLE => {
                let record = sample::Body::from_ptr(
                    follow_mem_ptr,
                    sampling.is_sample_stack_user,
                    sampling.is_sample_callchain,
                    sampling.is_sample_aux,
                    sampling.user_regs_len,
                    sampling.intr_regs_len,
                );
                RecordBody::Sample(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_MMAP2 => {
                let record = mmap2::Body::from_ptr(
                    follow_mem_ptr,
                    record_header.misc,
                    sampling.is_sample_id_all,
                );
                RecordBody::Mmap2(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_AUX => {
                let record = aux::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Aux(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_ITRACE_START => {
                let ptr = follow_mem_ptr as *const intrace_start::Body;
                RecordBody::ItraceStart(ptr.read().wrap_box())
            }
            perf_event_type_PERF_RECORD_LOST_SAMPLES => {
                let record =
                    lost_samples::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::LostSamples(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_SWITCH => {
                let record = switch::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Switch(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_SWITCH_CPU_WIDE => {
                let record =
                    switch_cpu_wide::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::SwitchCpuWide(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_NAMESPACES => {
                let record = namespaces::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Namespaces(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_KSYMBOL => {
                let record = ksymbol::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Ksymbol(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_BPF_EVENT => {
                let record = bpf_event::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::BpfEvent(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_CGROUP => {
                let record = cgroup::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::Cgroup(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_TEXT_POKE => {
                let record = text_poke::Body::from_ptr(follow_mem_ptr, sampling.is_sample_id_all);
                RecordBody::TextPoke(record.wrap_box())
            }
            perf_event_type_PERF_RECORD_AUX_OUTPUT_HW_ID => {
                let ptr = follow_mem_ptr as *const aux_output_hw_id::Body;
                RecordBody::AuxOutputHwId(ptr.read().wrap_box())
            }
            _ => unreachable!(),
        }
    };

    //dbg!(record_header.size - 8);
    //dbg!(record_body);
    //return None;

    Record {
        misc: record_header.misc,
        body: record_body,
    }
    .wrap_some()
}
