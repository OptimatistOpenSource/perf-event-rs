/*
struct {
  u32    pid;
  u32    tid;
  char   comm[];
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    pid: u32,
    tid: u32,
    // TODO
    sample_id: sample_id,
}
