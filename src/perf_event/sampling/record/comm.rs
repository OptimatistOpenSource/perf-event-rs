/*
struct {
  u32    pid;
  u32    tid;
  char   comm[];
  struct sample_id sample_id;
};
*/

use crate::infra::NullTerminated;
use crate::sampling::record::sample_id;

#[repr(C)]
pub(crate) struct Body {
    pid: u32,
    tid: u32,
    comm: NullTerminated<u8>,
    sample_id: sample_id,
}
