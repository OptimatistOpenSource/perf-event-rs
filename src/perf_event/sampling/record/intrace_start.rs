/*
struct {
  u32    pid;
  u32    tid;
};
*/

#[repr(C)]
pub struct Body {
    pid: u32,
    tid: u32,
}
