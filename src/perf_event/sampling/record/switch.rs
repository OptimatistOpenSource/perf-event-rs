/*
struct {
  struct sample_id sample_id;
};
*/

use crate::sampling::record::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Body {
    pub sample_id: SampleId,
}
