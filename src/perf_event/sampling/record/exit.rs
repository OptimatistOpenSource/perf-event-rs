/*
struct {
  u32    pid, ppid;
  u32    tid, ptid;
  u64    time;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    pid: u32,
    ppid: u32,
    tid: u32,
    ptid: u32,
    time: u64,
    sample_id: sample_id,
}
