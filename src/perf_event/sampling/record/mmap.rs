/*
struct {
  u32    pid, tid;
  u64    addr;
  u64    len;
  u64    pgoff;
  char   filename[];
};
*/

#[repr(C)]
pub struct Body {
    pid: u32,
    tid: u32,
    addr: u64,
    len: u64,
    pgoff: u64,
    // TODO
}
