/*
struct {
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
#[derive(Debug)]
pub struct Body {
    pub sample_id: sample_id,
}
