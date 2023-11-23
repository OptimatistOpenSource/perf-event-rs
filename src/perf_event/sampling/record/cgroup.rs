/*
struct {
  u64    id;
  char   path[];
  struct sample_id sample_id;
};
*/

use crate::infra::NullTerminated;
use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    id: u64,
    path: NullTerminated<u8>,
    sample_id: sample_id,
}
