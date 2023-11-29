/*
struct {
  u32 next_prev_pid;
  u32 next_prev_tid;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
#[derive(Debug)]
pub struct Body {
    pub next_prev_pid: u32,
    pub next_prev_tid: u32,
    pub sample_id: sample_id,
}
