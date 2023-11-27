/*
struct {
  u32    pid, tid;
  struct read_format values;
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub(crate) struct Body {
    pid: u32,
    tid: u32,
    // TODO
    sample_id: sample_id,
}
