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
#[derive(Debug)]
pub struct Body {
    pub pid: u32,
    pub ppid: u32,
    pub tid: u32,
    pub ptid: u32,
    pub time: u64,
    pub sample_id: sample_id,
}
