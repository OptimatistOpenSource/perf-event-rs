/*
struct {
  u32    pid;
  u32    tid;
};
*/

#[repr(C)]
#[derive(Debug)]
pub struct Body {
    pub pid: u32,
    pub tid: u32,
}
