/*
struct {
  u64    id;
  char   path[];
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    id: u64,
    // TODO
    sample_id: sample_id,
}
