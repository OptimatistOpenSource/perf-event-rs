/*
struct {
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Raw;

impl Raw {
    pub unsafe fn sample_id(&self, sample_type: u64) -> SampleId {
        let ptr = self as *const _ as _;
        SampleId::from_ptr(ptr, sample_type)
    }
}
