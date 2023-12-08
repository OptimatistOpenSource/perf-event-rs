/*
struct {
  u64    time;
  u64    id;
  u64    stream_id;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Body {
    pub time: u64,
    pub id: u64,
    pub stream_id: u64,
}

impl Body {
    pub unsafe fn sample_id(&self) -> &SampleId {
        let ptr = (self as *const Self).add(1) as *const SampleId;
        ptr.as_ref().unwrap()
    }
}
