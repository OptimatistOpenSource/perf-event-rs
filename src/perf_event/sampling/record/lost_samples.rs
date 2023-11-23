/*
struct {
  u64    lost;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    lost: u64,
    sample_id: sample_id,
}
