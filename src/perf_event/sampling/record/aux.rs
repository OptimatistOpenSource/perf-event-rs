/*
struct {
  u64    aux_offset;
  u64    aux_size;
  u64    flags;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Body {
    pub aux_offset: u64,
    pub aux_size: u64,
    pub flags: u64,
    pub sample_id: SampleId,
}
