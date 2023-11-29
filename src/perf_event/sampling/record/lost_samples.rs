/*
struct {
  u64    lost;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
#[derive(Debug)]
pub struct Body {
    pub lost: u64,
    pub sample_id: sample_id,
}
