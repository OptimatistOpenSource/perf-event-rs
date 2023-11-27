/*
struct {
  u64    time;
  u64    id;
  u64    stream_id;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub(crate) struct Body {
    time: u64,
    id: u64,
    stream_id: u64,
    sample_id: sample_id,
}
