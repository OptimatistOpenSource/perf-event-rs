mod attr;
mod builder;
mod record;
#[cfg(test)]
mod tests;

use crate::infra::{ArrayExt, VecExt, WrapOption, WrapResult};
use crate::sampling::record::{sample, throttle, unthrottle, Record, RecordBody};
use crate::syscall;
use crate::syscall::bindings::*;
use crate::syscall::{ioctl_wrapped, perf_event_open};
pub use attr::*;
pub use builder::*;
use memmap::{MmapMut, MmapOptions};
use std::fs::File;
use std::os::fd::FromRawFd;
use std::{io, slice};

pub struct Sampling {
    pub(crate) mmap: MmapMut,
    pub(crate) file: File,
}

// TODO: impl iter
impl Sampling {
    pub(crate) unsafe fn new(
        attr: &Attr,
        pid: i32,
        cpu: i32,
        group_fd: i32,
        flags: u64,
    ) -> io::Result<Self> {
        let i32 = unsafe { perf_event_open(attr.as_raw(), pid, cpu, group_fd, flags) };
        match i32 {
            -1 => Err(io::Error::last_os_error()),
            fd => {
                let file = File::from_raw_fd(fd);
                let pages = 1 + (1 << 16); // TODO
                let mmap = unsafe {
                    MmapOptions::new()
                        .len(page_size::get() * pages)
                        .map_mut(&file)
                }
                .unwrap();

                Self { mmap, file }
            }
            .wrap_ok(),
        }
    }

    pub fn enable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(
            &self.file,
            syscall::bindings::perf_event_ioctls_ENABLE,
            None,
        )
    }

    pub fn disable(&self) -> io::Result<()> {
        ioctl_wrapped::<()>(
            &self.file,
            syscall::bindings::perf_event_ioctls_DISABLE,
            None,
        )
    }

    // TODO
    pub fn next_sample(&mut self) -> Option<Record> {
        let metapage =
            unsafe { (self.mmap.as_mut_ptr() as *mut perf_event_mmap_page).as_mut() }.unwrap();

        dbg!(metapage.data_tail);
        dbg!(metapage.data_head);

        if metapage.data_tail == metapage.data_head {
            return None;
        }

        let ring_ptr = unsafe { self.mmap.as_mut_ptr().offset(metapage.data_offset as _) };

        let record_len = match metapage.data_tail as isize + 8 - metapage.data_size as isize {
            left if left <= 0 => {
                let offset = (metapage.data_tail + 6) as isize;
                let ptr = unsafe { ring_ptr.offset(offset) } as *const u16;
                unsafe { *ptr }
            }
            1 => unsafe {
                let mut buf = <[u8; 2]>::uninit();
                buf[0] = *(ring_ptr.offset((metapage.data_size - 1) as isize) as *const u8);
                buf[1] = *(ring_ptr.offset(0) as *const u8);
                std::mem::transmute(buf)
            },
            left => unsafe {
                let ptr = ring_ptr.offset(left - 2) as *const u16;
                *ptr
            },
        } as usize;

        dbg!(record_len);

        let record_buf =
            match metapage.data_tail as isize + record_len as isize - metapage.data_size as isize {
                left if left > 0 => {
                    let ring_end_part = {
                        let start = metapage.data_tail as isize;
                        let len = (metapage.data_size - metapage.data_tail) as usize;
                        unsafe { slice::from_raw_parts(ring_ptr.offset(start), len) }
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
                _ => unsafe {
                    slice::from_raw_parts(ring_ptr.offset(metapage.data_tail as _), record_len)
                }
                .to_vec(),
            };

        let record_header =
            unsafe { (record_buf.as_ptr() as *const perf_event_header).as_ref() }.unwrap();
        dbg!(record_header.type_);

        #[allow(non_upper_case_globals)]
        let record_body = unsafe {
            let follow_mem_ptr = (record_header as *const perf_event_header).offset(1) as *const _;
            match record_header.type_ {
                /*
                (perf_event_type_PERF_RECORD_MMAP,Mmap,mmap::Body),
                (perf_event_type_PERF_RECORD_LOST,Lost,lost::Body),
                (perf_event_type_PERF_RECORD_COMM,Comm,comm::Body),
                (perf_event_type_PERF_RECORD_EXIT,Exit,exit::Body),
                */
                perf_event_type_PERF_RECORD_THROTTLE => {
                    let ptr = follow_mem_ptr as *const throttle::Body;
                    RecordBody::Throttle(ptr.read())
                }
                perf_event_type_PERF_RECORD_UNTHROTTLE => {
                    let ptr = follow_mem_ptr as *const unthrottle::Body;
                    RecordBody::Unthrottle(ptr.read())
                }
                /*
                (perf_event_type_PERF_RECORD_FORK,Fork,fork::Body),
                (perf_event_type_PERF_RECORD_READ,Read,read::Body),
                */
                perf_event_type_PERF_RECORD_SAMPLE => {
                    RecordBody::Sample(sample::Body::from_ptr(follow_mem_ptr))
                }
                /*
                (perf_event_type_PERF_RECORD_MMAP2,Mmap2,mmap2::Body),
                (perf_event_type_PERF_RECORD_AUX,Aux,aux::Body),
                (perf_event_type_PERF_RECORD_ITRACE_START,ItraceStart,intrace_start::Body),
                (perf_event_type_PERF_RECORD_LOST_SAMPLES,LostSamples,lost_samples::Body),
                (perf_event_type_PERF_RECORD_SWITCH,Switch,switch::Body),
                (perf_event_type_PERF_RECORD_SWITCH_CPU_WIDE,SwitchCpuWide,switch_cpu_wide::Body),
                (perf_event_type_PERF_RECORD_NAMESPACES,Namespaces,namespaces::Body),
                (perf_event_type_PERF_RECORD_KSYMBOL,Ksymbol,ksymbol::Body),
                (perf_event_type_PERF_RECORD_BPF_EVENT,BpfEvent,bpf_event::Body),
                (perf_event_type_PERF_RECORD_CGROUP,Cgroup,cgroup::Body),
                (perf_event_type_PERF_RECORD_TEXT_POKE,TextPoke,text_poke::Body),
                (perf_event_type_PERF_RECORD_AUX_OUTPUT_HW_ID,AuxOutputHwId,aux_output_hw_id::Body),
                */
                _ => todo!(),
            }
        };

        match metapage.data_tail as isize + record_header.size as isize
            - metapage.data_size as isize
        {
            offset if offset < 0 => metapage.data_tail += record_header.size as u64,
            offset => metapage.data_tail = offset as _,
        }

        Record {
            misc: record_header.misc,
            body: record_body,
        }
        .wrap_some()
    }
}
