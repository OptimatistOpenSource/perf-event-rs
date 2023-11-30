/*
struct {
  u32    pid, tid;
  struct read_format values;
  struct sample_id sample_id;
};
*/

use crate::counting::{read_format_body, read_format_header};
use crate::infra::{ConstPtrExt, SliceExt};
use crate::sampling::record::sample_id;
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
        pub fn $name(&self) -> $ty {
            &self.sized1().$name
        }
    };
}

impl Body {
    fn sized1(&self) -> &Sized1 {
        let ptr = self as *const _ as *const Sized1;
        unsafe { ptr.as_ref().unwrap() }
    }
    sized1_get!(pid, &u32);
    sized1_get!(tid, &u32);
    sized1_get!(values_header, &read_format_header);

    pub fn values_body(&self) -> &[read_format_body] {
        let sized1_ptr = self.sized1() as *const Sized1;
        let ptr = unsafe { sized1_ptr.add(1).align_as_ptr::<read_format_body>() };
        let members_len = self.values_header().members_len as usize;
        unsafe { slice::from_raw_parts(ptr, members_len) }
    }

    pub fn sample_id(&self) -> &sample_id {
        let ptr = unsafe {
            self.values_body()
                .follow_mem_ptr()
                .align_as_ptr::<sample_id>()
        };
        unsafe { ptr.as_ref() }.unwrap()
    }
}
