/*
struct {
  u64    id;
  char   path[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::sample_id::SampleId;

pub struct Raw {
    pub read_ptr: *const u8,
    pub sample_type: u64,
}

impl Raw {
    pub unsafe fn id(&mut self) -> &u64 {
        let ptr = self.read_ptr as *const u64;
        self.read_ptr = ptr.add(1) as _;
        ptr.as_ref().unwrap()
    }

    pub unsafe fn path(&mut self) -> &[u8] {
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
