/*
struct {
  u64    sample_id;   /* if PERF_SAMPLE_IDENTIFIER */
  u64    ip;          /* if PERF_SAMPLE_IP */
  u32    pid, tid;    /* if PERF_SAMPLE_TID */
  u64    time;        /* if PERF_SAMPLE_TIME */
  u64    addr;        /* if PERF_SAMPLE_ADDR */
  u64    id;          /* if PERF_SAMPLE_ID */
  u64    stream_id;   /* if PERF_SAMPLE_STREAM_ID */
  u32    cpu, res;    /* if PERF_SAMPLE_CPU */
  u64    period;      /* if PERF_SAMPLE_PERIOD */
  struct read_format v;
                      /* if PERF_SAMPLE_READ */
  u64    nr;          /* if PERF_SAMPLE_CALLCHAIN */
  u64    ips[nr];     /* if PERF_SAMPLE_CALLCHAIN */
  u32    size;        /* if PERF_SAMPLE_RAW */
  char   data[size];  /* if PERF_SAMPLE_RAW */
  u64    bnr;         /* if PERF_SAMPLE_BRANCH_STACK */
  struct perf_branch_entry lbr[bnr];
                      /* if PERF_SAMPLE_BRANCH_STACK */
  u64    abi;         /* if PERF_SAMPLE_REGS_USER */
  u64    regs[weight(mask)];
                      /* if PERF_SAMPLE_REGS_USER */
  u64    size;        /* if PERF_SAMPLE_STACK_USER */
  char   data[size];  /* if PERF_SAMPLE_STACK_USER */
  u64    dyn_size;    /* if PERF_SAMPLE_STACK_USER &&
                         size != 0 */
  union perf_sample_weight weight;
                      /* if PERF_SAMPLE_WEIGHT */
                      /* || PERF_SAMPLE_WEIGHT_STRUCT */
  u64    data_src;    /* if PERF_SAMPLE_DATA_SRC */
  u64    transaction; /* if PERF_SAMPLE_TRANSACTION */
  u64    abi;         /* if PERF_SAMPLE_REGS_INTR */
  u64    regs[weight(mask)];
                      /* if PERF_SAMPLE_REGS_INTR */
  u64    phys_addr;   /* if PERF_SAMPLE_PHYS_ADDR */
  u64    cgroup;      /* if PERF_SAMPLE_CGROUP */
  u64    data_page_size;
                    /* if PERF_SAMPLE_DATA_PAGE_SIZE */
  u64    code_page_size;
                    /* if PERF_SAMPLE_CODE_PAGE_SIZE */
  u64    size;        /* if PERF_SAMPLE_AUX */
  char   data[size];  /* if PERF_SAMPLE_AUX */
};
*/

use crate::infra::{ConstPtrExt, SizedExt, SliceExt, Vla, WrapOption};
use crate::syscall::bindings::*;
use std::ops::Not;
use std::slice;

pub struct Body {
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

impl Body {
    #[inline]
    const fn is_enabled(&self, mask: Mask) -> bool {
        (self.sample_type & mask as u64) > 0
    }

    #[inline]
    unsafe fn get_if<T>(&mut self, mask: Mask) -> Option<&T> {
        if self.is_enabled(mask).not() {
            return None;
        }
        let ptr = self.read_ptr as *const T;
        self.read_ptr = self.read_ptr.add(T::size());
        ptr.as_ref()
    }

    gen_fn! { u64, sample_id PERF_SAMPLE_IDENTIFIER }
    gen_fn! { u64, ip        PERF_SAMPLE_IP         }
    gen_fn! { u32, pid       PERF_SAMPLE_TID        }
    gen_fn! { u32, tid       PERF_SAMPLE_TID        }
    gen_fn! { u64, time      PERF_SAMPLE_TIME       }
    gen_fn! { u64, addr      PERF_SAMPLE_ADDR       }
    gen_fn! { u64, id        PERF_SAMPLE_ID         }
    gen_fn! { u64, stream_id PERF_SAMPLE_STREAM_ID  }

    pub unsafe fn cpu(&mut self) -> Option<&u32> {
        if self.is_enabled(PERF_SAMPLE_CPU).not() {
            return None;
        }

        let cpu_ptr = self.read_ptr as *const u32;
        self.read_ptr = cpu_ptr.add(2) as _; // skip 32-bit res
        cpu_ptr.as_ref()
    }

    gen_fn! { u64, period    PERF_SAMPLE_PERIOD     }

    pub unsafe fn v(&mut self) -> Option<(&read_format_header, &[read_format_body])> {
        if self.is_enabled(PERF_SAMPLE_READ).not() {
            return None;
        }

        let header_ptr = self.read_ptr as *const read_format_header;
        let header = header_ptr.as_ref().unwrap();

        let body_ptr = header_ptr.add(1) as *const read_format_body;
        let slice = slice::from_raw_parts(body_ptr, header.members_len as _);

        self.read_ptr = slice.follow_mem_ptr() as _;

        (header, slice).wrap_some()
    }

    pub unsafe fn ips(&mut self) -> Option<&[u64]> {
        if self.is_enabled(PERF_SAMPLE_CALLCHAIN).not() {
            return None;
        }
        let len_ptr = self.read_ptr as *const u64;
        let vla: &Vla<u64, u64> = Vla::from_ptr(len_ptr).as_ref().unwrap();
        let slice = vla.as_slice();
        self.read_ptr = slice.follow_mem_ptr() as _;
        slice.wrap_some()
    }

    pub unsafe fn data_raw(&mut self) -> Option<&[u8]> {
        if self.is_enabled(PERF_SAMPLE_RAW).not() {
            return None;
        }
        let len_ptr = self.read_ptr as *const u32;
        // The values are padded with 0 (in the end) to have 64-bit alignment.
        let values_ptr = len_ptr.add(1) as *const u8;
        let slice = slice::from_raw_parts(values_ptr, *len_ptr as _);
        self.read_ptr = slice.follow_mem_ptr() as _;
        slice.wrap_some()
    }

    /*
    TODO: if PERF_SAMPLE_BRANCH_STACK
    u64    bnr;
    struct perf_branch_entry lbr[bnr];
    */

    pub unsafe fn abi_and_regs_user(&mut self, regs_len: usize) -> Option<(&u64, &[u64])> {
        if self.is_enabled(PERF_SAMPLE_REGS_USER).not() {
            return None;
        }

        let abi_ptr = self.read_ptr as *const u64;
        let regs_ptr = abi_ptr.add(1);
        /*
        From line 7387 of kernel/events/core.c:
        If there are no regs to dump, notice it through
        first u64 being zero (PERF_SAMPLE_REGS_ABI_NONE).
        */
        let abi = abi_ptr.as_ref().unwrap();
        let regs_len = if *abi == PERF_SAMPLE_REGS_ABI_NONE as _ {
            0
        } else {
            regs_len
        };
        let regs = slice::from_raw_parts(regs_ptr, regs_len);
        self.read_ptr = regs.follow_mem_ptr() as _;
        (abi, regs).wrap_some()
    }

    pub unsafe fn data_stack_user(&mut self) -> Option<&[u8]> {
        if self.is_enabled(PERF_SAMPLE_STACK_USER).not() {
            return None;
        }

        let len_ptr = self.read_ptr as *const u64;
        let vla: &Vla<u64, u8> = unsafe { Vla::from_ptr(len_ptr).as_ref().unwrap() };
        let slice = vla.as_slice();
        let u64_aligned_ptr = slice.follow_mem_ptr().align_as_ptr::<u64>();

        if *len_ptr == 0 {
            self.read_ptr = u64_aligned_ptr as _;
            slice.wrap_some()
        } else {
            let dyn_size_ptr = u64_aligned_ptr;
            self.read_ptr = dyn_size_ptr.add(1) as _;
            slice[..*dyn_size_ptr as _].wrap_some()
        }
    }

    pub unsafe fn weight(&mut self) -> Option<&perf_sample_weight> {
        if (self.is_enabled(PERF_SAMPLE_WEIGHT) || self.is_enabled(PERF_SAMPLE_WEIGHT_STRUCT)).not()
        {
            return None;
        }

        let ptr = self.read_ptr as *const perf_sample_weight;
        self.read_ptr = self.read_ptr.add(perf_sample_weight::size());
        ptr.as_ref()
    }

    gen_fn! { u64, data_src    PERF_SAMPLE_DATA_SRC    }
    gen_fn! { u64, transaction PERF_SAMPLE_TRANSACTION }

    pub unsafe fn abi_and_regs_intr(&mut self, regs_len: usize) -> Option<(&u64, &[u64])> {
        if self.is_enabled(PERF_SAMPLE_REGS_INTR).not() {
            return None;
        }

        let abi_ptr = self.read_ptr as *const u64;
        let regs_ptr = abi_ptr.add(1);
        /*
        From line 7387 of kernel/events/core.c:
        If there are no regs to dump, notice it through
        first u64 being zero (PERF_SAMPLE_REGS_ABI_NONE).
        */
        let abi = abi_ptr.as_ref().unwrap();
        let regs_len = if *abi == PERF_SAMPLE_REGS_ABI_NONE as _ {
            0
        } else {
            regs_len
        };
        let regs = slice::from_raw_parts(regs_ptr, regs_len);
        self.read_ptr = regs.follow_mem_ptr() as _;
        (abi, regs).wrap_some()
    }

    gen_fn! { u64, phys_addr      PERF_SAMPLE_PHYS_ADDR      }
    gen_fn! { u64, cgroup         PERF_SAMPLE_CGROUP         }
    gen_fn! { u64, data_page_size PERF_SAMPLE_DATA_PAGE_SIZE }
    gen_fn! { u64, code_page_size PERF_SAMPLE_CODE_PAGE_SIZE }

    /*
    TODO: if PERF_SAMPLE_AUX
    u64    size;
    char   data[size];
    */
}
