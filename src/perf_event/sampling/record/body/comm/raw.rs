/*
struct {
  u32    pid;
  u32    tid;
  char   comm[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::sample_id::SampleId;

#[repr(C)]
pub struct Sized {
    pub pid: u32,
    pub tid: u32,
}

pub struct Raw {
    pub read_ptr: *const u8,
    pub sample_type: u64,
}

// TODO: simplify: ptr.as_ref().unwrap()
impl Raw {
    pub unsafe fn sized(&mut self) -> &Sized {
        let ptr = self.read_ptr as *const Sized;
        self.read_ptr = ptr.add(1) as _;
        ptr.as_ref().unwrap()
    }

    pub unsafe fn comm(&mut self) -> &[u8] {
        let ptr = self.read_ptr;
        let zt = ZeroTerminated::from_ref(ptr.as_ref().unwrap());
        let slice = zt.as_slice();
        // Above [u8] will be rounded up to 64-bit in size in the kernel
        self.read_ptr = slice.follow_mem_ptr().align_as_ptr::<u64>() as _;
        slice
    }

    pub unsafe fn sample_id(&self) -> SampleId {
        SampleId::from_ptr(self.read_ptr, self.sample_type)
    }
}
