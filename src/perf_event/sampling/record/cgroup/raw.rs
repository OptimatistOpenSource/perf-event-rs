/*
struct {
  u64    id;
  char   path[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::SampleId;

#[repr(C)]
pub struct Body;

impl Body {
    pub fn id(&self) -> &u64 {
        let ptr = self as *const _ as *const u64;
        unsafe { ptr.as_ref() }.unwrap()
    }

    pub fn path(&self) -> &[u8] {
        let ptr = unsafe { (self.id() as *const u64).add(1) } as *const u8;
        let zt = unsafe { ZeroTerminated::from_ref(ptr.as_ref().unwrap()) };
        zt.as_slice()
    }

    pub fn sample_id(&self) -> &SampleId {
        let ptr = unsafe { self.path().follow_mem_ptr().align_as_ptr::<SampleId>() };
        unsafe { ptr.as_ref() }.unwrap()
    }
}
