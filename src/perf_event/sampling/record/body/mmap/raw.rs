/*
struct {
  u32    pid, tid;
  u64    addr;
  u64    len;
  u64    pgoff;
  char   filename[];
};
*/

use crate::infra::ZeroTerminated;

#[repr(C)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
    pub addr: u64,
    pub len: u64,
    pub pgoff: u64,
    pub filename: ZeroTerminated<u8>,
}
