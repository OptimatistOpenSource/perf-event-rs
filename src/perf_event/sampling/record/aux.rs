/*
struct {
  u64    aux_offset;
  u64    aux_size;
  u64    flags;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    aux_offset: u64,
    aux_size: u64,
    flags: u64,
    sample_id: sample_id,
}
