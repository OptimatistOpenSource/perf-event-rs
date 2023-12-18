/*
struct {
  u32    pid, tid;
  struct read_format values;
  struct sample_id sample_id;
};
*/

use crate::infra::SliceExt;
use crate::sampling::record::sample_id::SampleId;
use crate::syscall::bindings::{read_format_body, read_format_header};
use std::slice;

#[repr(C)]
pub struct Sized {
    pub pid: u32,
    pub tid: u32,
}

pub struct Raw {
    pub read_ptr: *const u8,
    pub sample_type: u64,
}

impl Raw {
    pub unsafe fn sized(&mut self) -> &Sized {
        let ptr = self.read_ptr as *const Sized;
        self.read_ptr = ptr.add(1) as _;
        ptr.as_ref().unwrap()
    }

    pub unsafe fn values(&mut self) -> (&read_format_header, &[read_format_body]) {
        let header_ptr = self.read_ptr as *const read_format_header;
        let header = header_ptr.as_ref().unwrap();
        let body_ptr = header_ptr.add(1) as *const read_format_body;
        let slice = slice::from_raw_parts(body_ptr, header.members_len as _);
        self.read_ptr = slice.follow_mem_ptr() as _;
        (header, slice)
    }

    pub unsafe fn sample_id(&self) -> SampleId {
        SampleId::from_ptr(self.read_ptr, self.sample_type)
    }
}
