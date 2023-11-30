/*
struct {
  u32    pid, tid;
  struct read_format values;
  struct sample_id sample_id;
};
*/

use crate::counting::{read_format_body, read_format_header};
use crate::infra::SliceExt;
use crate::sampling::record::SampleId;
use std::slice;

#[repr(C)]
struct Sized1 {
    pub pid: u32,
    pub tid: u32,
    pub values_header: read_format_header,
}

#[repr(C)]
pub struct Body;

macro_rules! sized1_get {
    ($name:ident,$ty:ty) => {
        #[inline]
        pub fn $name(&self) -> $ty {
            &self.sized1().$name
        }
    };
}

impl Body {
    #[inline]
    fn sized1(&self) -> &Sized1 {
        let ptr = self as *const _ as *const Sized1;
        unsafe { ptr.as_ref().unwrap() }
    }
    sized1_get!(pid, &u32);
    sized1_get!(tid, &u32);
    sized1_get!(values_header, &read_format_header);

    pub fn values_body(&self) -> &[read_format_body] {
        let sized1_ptr = self.sized1() as *const Sized1;
        let ptr = unsafe { sized1_ptr.add(1) as *const read_format_body };
        let members_len = self.values_header().members_len as usize;
        unsafe { slice::from_raw_parts(ptr, members_len) }
    }

    pub fn sample_id(&self) -> &SampleId {
        let ptr = unsafe { self.values_body().follow_mem_ptr() as *const SampleId };
        unsafe { ptr.as_ref() }.unwrap()
    }
}
