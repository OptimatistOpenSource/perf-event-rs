/*
struct {
  u32 next_prev_pid;
  u32 next_prev_tid;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    next_prev_pid: u32,
    next_prev_tid: u32,
    sample_id: sample_id,
}
