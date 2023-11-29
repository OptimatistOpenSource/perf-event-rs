/*
struct {
  u64    id;
  char   path[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body;

impl Body {
    pub fn id(&self) -> &u64 {
        let ptr = self as *const _ as *const u64;
        unsafe { ptr.as_ref() }.unwrap()
    }

    pub fn path(&self) -> &ZeroTerminated<u8> {
        let ptr = unsafe { (self.id() as *const u64).add(1) } as *const u8;
        unsafe { ZeroTerminated::from_ref(ptr.as_ref().unwrap()) }
    }

    pub fn sample_id(&self) -> &sample_id {
        let ptr = unsafe {
            self.path()
                .as_slice()
                .follow_mem_ptr()
                .align_as_ptr::<sample_id>()
        };
        unsafe { ptr.as_ref() }.unwrap()
    }
}
