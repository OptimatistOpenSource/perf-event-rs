/*
struct {
  u64    addr;
  u32    len;
  u16    ksym_type;
  u16    flags;
  char   name[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::sample_id::SampleId;

#[repr(C)]
pub struct Sized {
    pub addr: u64,
    pub len: u32,
    pub ksym_type: u16,
    pub flags: u16,
}

pub struct Raw {
    pub read_ptr: *const u8,
    pub sample_type: u64,
}

// TODO: use read_ptr
impl Raw {
    pub unsafe fn sized(&mut self) -> &Sized {
        let ptr = self.read_ptr as *const Sized;
        self.read_ptr = ptr.add(1) as _;
        ptr.as_ref().unwrap()
    }

    pub unsafe fn name(&mut self) -> &[u8] {
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
