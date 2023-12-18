/*
struct {
  u64    aux_offset;
  u64    aux_size;
  u64    flags;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Raw {
    pub aux_offset: u64,
    pub aux_size: u64,
    pub flags: u64,
}

impl Raw {
    pub unsafe fn sample_id(&self, sample_type: u64) -> SampleId {
        let ptr = (self as *const Self).add(1) as _;
        SampleId::from_ptr(ptr, sample_type)
    }
}
