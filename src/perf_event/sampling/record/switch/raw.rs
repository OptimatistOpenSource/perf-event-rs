/*
struct {
  struct sample_id sample_id;
};
*/

use crate::sampling::record::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Body;

impl Body {
    pub unsafe fn sample_id(&self) -> &SampleId {
        let ptr = self as *const Self as *const SampleId;
        ptr.as_ref().unwrap()
    }
}
