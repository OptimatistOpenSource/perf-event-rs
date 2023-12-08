/*
struct {
  u32 next_prev_pid;
  u32 next_prev_tid;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Body {
    pub next_prev_pid: u32,
    pub next_prev_tid: u32,
}

impl Body {
    pub unsafe fn sample_id(&self) -> &SampleId {
        let ptr = (self as *const Self).add(1) as *const SampleId;
        ptr.as_ref().unwrap()
    }
}
