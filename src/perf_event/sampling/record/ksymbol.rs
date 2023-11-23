/*
struct {
  u64    addr;
  u32    len;
  u16    ksym_type;
  u16    flags;
  char   name[];
  struct sample_id sample_id;
};
*/

use crate::infra::NullTerminated;
use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    addr: u64,
    len: u32,
    ksym_type: u16,
    flags: u16,
    name: NullTerminated<u8>,
    sample_id: sample_id,
}
