/*
struct {
  u32    pid, tid;
  u64    addr;
  u64    len;
  u64    pgoff;
  char   filename[];
};
*/

use crate::infra::NullTerminated;

#[repr(C)]
pub struct Body {
    pid: u32,
    tid: u32,
    addr: u64,
    len: u64,
    pgoff: u64,
    filename: NullTerminated<u8>,
}
