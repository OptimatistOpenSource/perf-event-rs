/*
struct {
  u16 type;
  u16 flags;
  u32 id;
  u8 tag[BPF_TAG_SIZE];
  struct sample_id sample_id;
};
*/

use crate::sampling::record::sample_id;

const BPF_TAG_SIZE: usize = 8; // TODO: use bindgen

#[repr(C)]
pub struct Body {
    r#type: u16,
    flags: u16,
    id: u32,
    tag: [u8; BPF_TAG_SIZE],
    sample_id: sample_id,
}
