/*
struct {
  u32    pid, ppid;
  u32    tid, ptid;
  u64    time;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::SampleId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Body {
    pub pid: u32,
    pub ppid: u32,
    pub tid: u32,
    pub ptid: u32,
    pub time: u64,
}

impl Body {
    pub unsafe fn sample_id(&self) -> &SampleId {
        let ptr = (self as *const Self).add(1) as *const SampleId;
        ptr.as_ref().unwrap()
    }
}
