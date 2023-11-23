/*
struct {
  u64    id;
  u64    lost;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    id: u64,
    lost: u64,
    sample_id: sample_id,
}
