/*
struct {
  u64    addr;
  u16    old_len;
  u16    new_len;
  u8     bytes[];
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

#[repr(C)]
pub struct Body {
    addr: u64,
    old_len: u16,
    new_len: u16,
    // TODO
    sample_id: sample_id,
}
