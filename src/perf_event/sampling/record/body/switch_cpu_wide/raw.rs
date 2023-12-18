/*
struct {
  u32 next_prev_pid;
  u32 next_prev_tid;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Raw {
    pub next_prev_pid: u32,
    pub next_prev_tid: u32,
}

impl Raw {
    pub unsafe fn sample_id(&self, sample_type: u64) -> SampleId {
        let ptr = (self as *const Self).add(1) as _;
        SampleId::from_ptr(ptr, sample_type)
    }
}
