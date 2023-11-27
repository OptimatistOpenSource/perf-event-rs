/*
struct {
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub(crate) struct Body {
    sample_id: sample_id,
}
