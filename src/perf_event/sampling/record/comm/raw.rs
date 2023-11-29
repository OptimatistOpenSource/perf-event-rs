/*
struct {
  u32    pid;
  u32    tid;
  char   comm[];
  struct sample_id sample_id;
};
*/

use crate::infra::{ConstPtrExt, SliceExt, ZeroTerminated};
use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body;

impl Body {
    pub fn pid(&self) -> &u32 {
        let ptr = self as *const _ as *const u32;
        unsafe { ptr.as_ref() }.unwrap()
    }

    pub fn tid(&self) -> &u32 {
        let ptr = unsafe { (self.pid() as *const u32).add(1) };
        unsafe { ptr.as_ref() }.unwrap()
    }

    pub fn comm(&self) -> &ZeroTerminated<u8> {
        let ptr = unsafe { (self.tid() as *const u32).add(1) } as *const u8;
        unsafe { ZeroTerminated::from_ref(ptr.as_ref().unwrap()) }
    }

    pub fn sample_id(&self) -> &sample_id {
        let ptr = unsafe {
            self.comm()
                .as_slice()
                .follow_mem_ptr()
                .align_as_ptr::<sample_id>()
        };
        unsafe { ptr.as_ref() }.unwrap()
    }
}
