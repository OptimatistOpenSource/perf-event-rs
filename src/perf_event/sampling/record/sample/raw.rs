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

use crate::counting::{read_format_body, read_format_header};
use crate::debug_struct_fn;
use crate::infra::{ConstPtrExt, SliceExt, Vla, WrapOption};
use std::fmt::{Debug, Formatter};
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct Sized1 {
    sample_id: u64,
    ip: u64,
    pid: u32,
    tid: u32,
    time: u64,
    addr: u64,
    id: u64,
    stream_id: u64,
    cpu: u32,
    res: u32,
    period: u64,
    v_header: read_format_header,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct Sized2 {
    // TODO:
    //weight: perf_sample_weight,
    pub data_src: u64,
    pub transaction: u64,
    // TODO:
    //abi_2: u64,
    //u64    regs[weight(mask)];
    pub phys_addr: u64,
    pub cgroup: u64,
    pub data_page_size: u64,
    pub code_page_size: u64,
}

#[repr(C)]
pub(crate) struct Body {}

macro_rules! sized1_get {
    ($name:ident,$ty:ty) => {
        pub fn $name(&self) -> $ty {
            &self.sized1().$name
        }
    };
}

macro_rules! sized2_get {
    ($name:ident,$ty:ty) => {
        pub fn $name(&self) -> $ty {
            &self.sized2().$name
        }
    };
}

impl Body {
    pub(crate) fn sized1(&self) -> &Sized1 {
        let ptr = self as *const _ as *const Sized1;
        unsafe { &*ptr }
    }
    sized1_get!(sample_id, &u64);
    sized1_get!(ip, &u64);
    sized1_get!(pid, &u32);
    sized1_get!(tid, &u32);
    sized1_get!(time, &u64);
    sized1_get!(addr, &u64);
    sized1_get!(id, &u64);
    sized1_get!(stream_id, &u64);
    sized1_get!(cpu, &u32);
    sized1_get!(res, &u32);
    sized1_get!(period, &u64);
    sized1_get!(v_header, &read_format_header);

    pub fn v_body(&self) -> &[read_format_body] {
        let sized1_ptr = self.sized1() as *const Sized1;
        let ptr = unsafe { sized1_ptr.offset(1).align_as_ptr::<read_format_body>() };
        let members_len = self.v_header().members_len as usize;
        unsafe { slice::from_raw_parts(ptr, members_len) }
    }

    pub fn ips(&self) -> &[u64] {
        let len_ptr = unsafe { self.v_body().follow_mem_ptr().align_as_ptr::<u64>() };
        let vla: &Vla<u64, u64> = unsafe { &*Vla::from_ptr(len_ptr) };
        vla.as_slice()
    }

    pub fn data_1(&self) -> &[u8] {
        let len_ptr = unsafe { self.ips().follow_mem_ptr().align_as_ptr::<u32>() };
        let vla: &Vla<u32, u8> = unsafe { &*Vla::from_ptr(len_ptr) };
        vla.as_slice()
    }

    // TODO:
    //bnr: u64,
    //lbr: Vla<u64, perf_branch_entry>,
    //abi_1: u64,
    //u64    regs[weight(mask)];

    pub fn data_2(&self) -> &[u8] {
        let len_ptr = unsafe { self.data_1().follow_mem_ptr().align_as_ptr::<u64>() };
        let vla: &Vla<u64, u8> = unsafe { &*Vla::from_ptr(len_ptr) };
        vla.as_slice()
    }

    pub fn dyn_size(&self) -> Option<&u64> {
        if self.data_2().is_empty() {
            None
        } else {
            let ptr = unsafe { self.data_2().follow_mem_ptr().align_as_ptr::<u64>() };
            unsafe { &*ptr }.wrap_some()
        }
    }

    pub(crate) fn sized2(&self) -> &Sized2 {
        let ptr = if let Some(dyn_size) = self.dyn_size() {
            let dyn_size_ptr = dyn_size as *const u64;
            unsafe { dyn_size_ptr.offset(1).align_as_ptr::<Sized2>() }
        } else {
            unsafe { self.data_2().follow_mem_ptr().align_as_ptr::<Sized2>() }
        };
        unsafe { &*ptr }
    }
    // TODO:
    //sized2_get!(weight, &perf_sample_weight);
    sized2_get!(data_src, &u64);
    sized2_get!(transaction, &u64);
    sized2_get!(phys_addr, &u64);
    sized2_get!(cgroup, &u64);
    sized2_get!(data_page_size, &u64);
    sized2_get!(code_page_size, &u64);

    pub fn data_3(&self) -> &[u8] {
        let sized2_ptr = self.sized2() as *const Sized2;
        let len_ptr = unsafe { sized2_ptr.offset(1).align_as_ptr::<u64>() };
        let vla: &Vla<u64, u8> = unsafe { &*Vla::from_ptr(len_ptr) };
        vla.as_slice()
    }
}

impl Debug for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        debug_struct_fn! {
            name: sample_body
            self: self
            fmt: f
            fields:
                sample_id
                ip
                pid
                tid
                time
                addr
                id
                stream_id
                cpu
                res
                period
                v_header
                v_body
                ips
                data_1
                data_2
                dyn_size
                // TODO:
                //weight
                data_src
                transaction
                phys_addr
                cgroup
                data_page_size
                code_page_size
                data_3
        }

        Ok(())
    }
}
